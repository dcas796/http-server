use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;
use uuid::Uuid;
use crate::HttpResult;
use crate::session::HttpSession;

#[derive(Debug, Clone)]
pub struct Server {
    port: u16,
    server_path: PathBuf,
    task_pool: Arc<RwLock<HashMap<Uuid, JoinHandle<()>>>>,
}

impl Server {
    pub async fn new(port: u16, server_path: PathBuf) -> HttpResult<Self> {
        println!("Starting server at http://localhost:{}", port);
        let socket = TcpListener::bind((Ipv4Addr::LOCALHOST, port)).await?;
        let task_pool = Arc::new(RwLock::new(HashMap::new()));
        {
            let server_path = server_path.clone();
            let task_pool_clone = task_pool.clone();
            task_pool.write().unwrap().insert(
                Uuid::new_v4(),
                tokio::spawn(async move { Self::listener_worker(socket, server_path, task_pool_clone).await })
            );
        }
        Ok(Self {
            port,
            server_path,
            task_pool,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Server {
    async fn listener_worker(socket: TcpListener, server_path: PathBuf, task_pool: Arc<RwLock<HashMap<Uuid, JoinHandle<()>>>>) {
        loop {
            match socket.accept().await {
                Ok((stream, addr)) => {
                    let mut lock = task_pool.write().unwrap();
                    let id = Uuid::new_v4();
                    let task_pool_clone = task_pool.clone();
                    let server_path = server_path.clone();
                    lock.insert(id, tokio::spawn(async move {
                        match Self::connection_worker(id, stream, addr, server_path).await {
                            Ok(_) => {}
                            Err(e) => eprintln!("[{id}] {e}"),
                        }
                        task_pool_clone.write().unwrap().remove(&id);
                    }));
                },
                Err(e) => eprintln!("Failed to accept connection: {}", e),
            }
        }
    }

    async fn connection_worker(id: Uuid, mut stream: TcpStream, addr: SocketAddr, server_path: PathBuf) -> HttpResult<()> {
        stream.set_nodelay(true)?;

        let mut session = HttpSession::new(id, addr, server_path);

        let mut buffer = [0u8; 1024];

        loop {
            let read_bytes = stream.read(&mut buffer).await?;
            if read_bytes == 0 { break; }

            let read_buffer = &buffer[..read_bytes];
            let response  = session.handle_request_bytes(read_buffer);
            stream.write_all(&response.into_bytes()).await?;
        }

        Ok(())
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        println!("Closing server...");
        for (_id, task) in self.task_pool.write().unwrap().drain() {
            task.abort();
        }
    }
}

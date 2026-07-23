#[cfg(feature = "gui")]
mod app;
#[cfg(feature = "gui")]
mod ext;
#[cfg(feature = "gui")]
mod error;

pub static SERVER_PATH: &str = "./public/";

#[cfg(all(feature = "gui", feature = "nogui"))]
compile_error!("feature \"gui\" and feature \"nogui\" cannot be enabled at the same time");

#[cfg(feature = "nogui")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use http_server_lib::server::Server;
    use tokio_util::codec::{FramedRead, LinesCodec};
    use futures::StreamExt;

    let _server = Server::new(8080, SERVER_PATH.into()).await?;
    let stdin = tokio::io::stdin();
    let mut reader = FramedRead::new(stdin, LinesCodec::new());
    loop {
        let line = reader.next().await.transpose()?.unwrap();
        if line == "q" {
            break;
        }
    }
    Ok(())
}

#[cfg(feature = "gui")]
fn main() -> iced::Result {
    use crate::app::HttpServerApp;
    iced::application(HttpServerApp::new, HttpServerApp::update, HttpServerApp::view)
        .title("Http Server App")
        .window_size((800, 600))
        .run()
}
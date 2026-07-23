use std::path::PathBuf;
use iced::{Center, Element, Fill, Task};
use iced::widget::{button, container, text, text_input};
use iced::widget::column;
use http_server_lib::server::Server;
use crate::error::AppError;
use crate::ext::TextExt;

#[derive(Debug)]
pub enum Message {
    StartServer,
    StartedServer(Option<Result<Server, AppError>>),
    PortInputChanged(String),
    StopServer,
}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::StartServer => Message::StartServer,
            Message::StartedServer(_) => Message::StartedServer(None),
            Message::PortInputChanged(input) => Message::PortInputChanged(input.clone()),
            Message::StopServer => Message::StopServer,
        }
    }
}

#[derive(Debug)]
enum ServerState {
    Stopped,
    Opening,
    Open(Server),
    Error(AppError),
}

pub struct HttpServerApp {
    port_input: String,

    server_state: ServerState,
}

impl HttpServerApp {
    pub fn new() -> Self {
        HttpServerApp {
            port_input: "8080".to_string(),
            server_state: ServerState::Stopped,
        }
    }
}

impl HttpServerApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PortInputChanged(input) => {
                self.port_input = input;
                Task::none()
            },
            Message::StartServer => {
                match self.port_input.parse() {
                    Ok(port) => {
                        self.server_state = ServerState::Opening;
                        Task::future(async move {
                            let result = Server::new(port, PathBuf::from(crate::main::SERVER_PATH)).await;
                            Message::StartedServer(Some(result.map_err(Into::into)))
                        })
                    },
                    Err(err) => {
                        self.server_state = ServerState::Error(err.into());
                        Task::none()
                    },
                }
            },
            Message::StartedServer(Some(result)) => {
                self.server_state = match result {
                    Ok(server) => ServerState::Open(server),
                    Err(err) => ServerState::Error(err),
                };
                Task::none()
            },
            Message::StartedServer(None) => {
                self.server_state = ServerState::Error(AppError::ServerCloneError);
                Task::none()
            }
            Message::StopServer => {
                /* If the server has been started, it will be stopped automatically when dropped. */
                self.server_state = ServerState::Stopped;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text("HTTP Server")
                    .size(30)
                    .bold(),
                column(match &self.server_state {
                    ServerState::Stopped => vec![
                        text("Server is stopped").into(),
                        text_input("Port number", &self.port_input)
                            .width(120)
                            .on_input(Message::PortInputChanged)
                            .on_submit(Message::StartServer)
                            .into(),
                        button("Start server").on_press(Message::StartServer).into(),
                    ],
                    ServerState::Open(server) => vec![
                        text!("Server running on port {}", server.port()).into(),
                        button("Stop Server").on_press(Message::StopServer).into()
                    ],
                    ServerState::Opening => vec![
                        text("Starting server...").into(),
                    ],
                    ServerState::Error(error) => vec![
                        text!("Error starting server: {error}").into(),
                        button("Reset").on_press(Message::StopServer).into()
                    ],
                })
                    .align_x(Center)
                    .spacing(10)
            ]
                .align_x(Center)
                .spacing(20)
        )
            .align_x(Center)
            .align_y(Center)
            .width(Fill)
            .height(Fill)
            .into()
    }
}

use tokio::net::TcpListener;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("{0}번 포트는 사용중입니다. (Port {0} already in use)")]
    PortInUse(u16)
}

struct Producer {
    name: String,
}

struct Consumer {
    name: String,
}

struct Channel {
    producers: Vec<Producer>,
    consumer: Vec<Consumer>,
}

struct Server {
    tcp_listener: TcpListener,
}

impl Server {
    async fn new(address: &'static str) -> Result<Self, ServerError> {
        let port = address.split(":").last().unwrap().parse::<u16>().unwrap();

        let tcp_listener = TcpListener::bind(address).await.map_err(|e| {
            if e.kind() == io::ErrorKind::AddrInUse {
                ServerError::PortInUse(port)
            } else {
                ServerError::Io(e)
            }
        })?;

        Ok(Server {
            tcp_listener,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), ServerError>{
    let address = "127.0.0.1:8080";
    #[allow(unused_variables)]
    let server = Server::new(address).await?;
    Ok(())
}


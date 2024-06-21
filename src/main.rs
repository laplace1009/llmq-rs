use std::collections::{HashMap, HashSet, VecDeque};
use tokio::net::TcpListener;
use std::io;
use std::ops::DerefMut;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("{0}번 포트는 사용중입니다. (Port {0} already in use)")]
    PortInUse(u16)
}

struct Message {
    message: Vec<u8>
}

struct Consumer {
    name: String,
}

struct Exchange {
    message_queues: HashMap<String, VecDeque<Vec<u8>>>
}

impl Exchange {
    fn new() -> Self {
        Exchange {
            message_queues: HashMap::new()
        }
    }
}

struct Channel {
    consumers: Vec<Consumer>,
    exchanges: HashMap<String, Exchange>,
}
impl Channel {
    fn new() -> Self {
        Channel {
            consumers: Vec::new(),
            exchanges: HashMap::new(),
        }
    }

    #[allow(unused)]
    async fn basic_publish(&mut self, exchange: String, routing_key: String, message: String) -> Result<(), io::Error> {
        if let Some(ex) = self.find_exchange(&exchange) {

        }
        todo!()
    }

    fn find_exchange(&mut self, exchange: &str) -> Option<&mut Exchange> {
        self.exchanges.get_mut(exchange)
    }

    fn find_queue<'a>(exchange: &'a mut Exchange, routing_key: &str) -> Option<&'a mut VecDeque<Vec<u8>>> {
        exchange.message_queues.get_mut(routing_key)
    }
}

struct Server {
    tcp_listener: TcpListener,
    channels: Vec<Channel>,
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
            channels: Vec::new(),
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


use serde::{Deserialize, Serialize};
use sysinfo::{Cpu, CpuExt, System, SystemExt};

use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tungstenite::Result;

#[derive(Serialize, Deserialize, Debug)]
struct CpuInfo {
    dt: String,
    #[serde(rename = "coreInfos")]
    core_infos: Vec<CoreInfo>,
}

impl CpuInfo {
    pub fn from(cpus: &[Cpu]) -> Self {
        Self {
            dt: "TODO".into(),
            core_infos: cpus.iter().map(|e| CoreInfo::from(e)).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CoreInfo {
    name: String,
    brand: String,
    usage: f32,
    frequency: u64,
}

impl CoreInfo {
    pub fn from(cpu: &Cpu) -> Self {
        Self {
            name: cpu.name().into(),
            brand: cpu.brand().into(),
            usage: cpu.cpu_usage(),
            frequency: cpu.frequency(),
        }
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    println!("New WebSocket connection: {}", peer);

    let mut sys = System::new();

    let mut interval = tokio::time::interval(std::time::Duration::from_millis(1000));

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        if msg?.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                sys.refresh_cpu(); // Refreshing CPU information.

                let cpu_info = CpuInfo::from(sys.cpus());

                ws_sender
                    .send(serde_json::to_string(&cpu_info).unwrap().into()) // FIXME unwrap usage
                    .await?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        println!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }
}

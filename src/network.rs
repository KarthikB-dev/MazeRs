use anyhow::{Context, Result};
use iroh::{endpoint::Connection, Endpoint, NodeAddr};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use base64::Engine;

use crate::game_types::Instruction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Packet {
    Moves(Vec<Instruction>),
}

pub struct NetworkManager {
    pub tx: mpsc::Sender<Packet>,
    pub rx: mpsc::Receiver<Packet>,
    pub player_id: u8,
}

// Start hosting and return the code immediately
pub async fn start_hosting() -> Result<(Endpoint, String)> {
    let endpoint: Endpoint = Endpoint::builder()
        .discovery_n0()
        .alpns(vec![b"tank-maze".to_vec()])
        .bind()
        .await?;

    let my_addr: NodeAddr = endpoint.node_addr().await?;

    // Encode as base64 for easier sharing
    let addr_json = serde_json::to_string(&my_addr)?;
    let addr_base64 = base64::engine::general_purpose::STANDARD.encode(addr_json.as_bytes());

    Ok((endpoint, addr_base64))
}

// Wait for a client to connect (called after showing the code)
pub async fn wait_for_client(endpoint: Endpoint) -> Result<NetworkManager> {
    // Accept connection
    let incoming = endpoint.accept().await.context("Failed to accept connection")?;
    let connecting = incoming.accept()?;
    let connection = connecting.await?;

    let manager = setup_network_tasks(connection, 0)?;

    Ok(manager)
}

pub async fn connect_as_client(host_code: String) -> Result<NetworkManager> {
    let endpoint: Endpoint = Endpoint::builder()
        .discovery_n0()
        .alpns(vec![b"tank-maze".to_vec()])
        .bind()
        .await?;

    // Decode from base64
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(host_code.trim())
        .context("Invalid base64 code")?;

    let json_str = String::from_utf8(decoded)
        .context("Invalid UTF-8 in decoded data")?;

    let addr: NodeAddr = serde_json::from_str(&json_str)
        .context("Invalid NodeAddr format")?;

    let connection: Connection = endpoint.connect(addr, b"tank-maze").await?;

    let manager = setup_network_tasks(connection, 1)?;

    Ok(manager)
}

fn setup_network_tasks(connection: Connection, player_id: u8) -> Result<NetworkManager> {
    let (game_tx, mut network_rx) = mpsc::channel::<Packet>(100);
    let (network_tx, game_rx) = mpsc::channel::<Packet>(100);

    // Sender Task
    let conn_clone = connection.clone();
    tokio::spawn(async move {
        while let Some(pkt) = network_rx.recv().await {
            let data = bincode::serialize(&pkt).unwrap();
            if let Ok(mut stream) = conn_clone.open_uni().await {
                if let Err(e) = stream.write_all(&data).await {
                    eprintln!("Failed to write to stream: {}", e);
                    continue;
                }
                if let Err(e) = stream.finish() {
                    eprintln!("Failed to finish stream: {}", e);
                }
            }
        }
    });

    // Receiver Task
    let conn_clone2 = connection.clone();
    let net_tx = network_tx.clone();
    tokio::spawn(async move {
        loop {
            match conn_clone2.accept_uni().await {
                Ok(mut stream) => {
                    let tx = net_tx.clone();
                    tokio::spawn(async move {
                        let mut buffer = vec![0u8; 1024];
                        match stream.read_exact(&mut buffer).await {
                            Ok(()) => {
                                if let Ok(pkt) = bincode::deserialize::<Packet>(&buffer) {
                                    let _ = tx.send(pkt).await;
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to read from stream: {}", e);
                            }
                        }
                    });
                }
                Err(_) => break,
            }
        }
    });

    Ok(NetworkManager {
        tx: game_tx,
        rx: game_rx,
        player_id,
    })
}

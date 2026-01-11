use anyhow::{Context, Result};
use iroh::{endpoint::Connection, Endpoint, NodeAddr};
use serde::{Deserialize, Serialize};
use serde_json;
use base64::{Engine, engine::general_purpose};
use tokio::sync::mpsc;

use crate::game_types::Instruction;

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    Moves(Vec<Instruction>),
}

pub struct NetworkManager {
    pub tx: mpsc::Sender<Packet>,
    pub rx: mpsc::Receiver<Packet>,
    pub player_id: u8,
}

pub async fn start_network() -> Result<NetworkManager> {
    println!("--- TANK MAZE P2P ---");

    // Create endpoint with ALPN protocol configured
    let endpoint = Endpoint::builder()
        .discovery_n0()
        .alpns(vec![b"tank-maze".to_vec()])
        .bind()
        .await?;

    let my_addr = endpoint.node_addr().await?;

    // Encode as base64 for easier copy-paste
    let addr_json = serde_json::to_string(&my_addr)?;
    println!("Are you (H)ost or (C)lient?");
    let mut mode_input = String::new();
    std::io::stdin().read_line(&mut mode_input)?;
    let is_host = mode_input.trim().eq_ignore_ascii_case("h");

    let connection: Connection = if is_host {
        let addr_base64 = general_purpose::STANDARD.encode(addr_json.as_bytes());
        println!("My Node Info (copy this code): {}", addr_base64);

        println!("Waiting for client...");
        let incoming = endpoint.accept().await.context("Wait failed")?;
        let connecting = incoming.accept()?;
        connecting.await?
    } else {
        println!("Enter Host Code (paste the base64 code from host):");
        let mut s = String::new();
        std::io::stdin().read_line(&mut s)?;

        // Decode from base64 first
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(s.trim())
            .context("Invalid base64 code")?;

        let json_str = String::from_utf8(decoded)
            .context("Invalid UTF-8 in decoded data")?;

        // Then deserialize from JSON
        let addr: NodeAddr = serde_json::from_str(&json_str)
            .context("Invalid NodeAddr format")?;
        endpoint.connect(addr, b"tank-maze").await?
    };

    println!("Connected!");

    // Host is 0, Client is 1
    let player_id = if is_host { 0 } else { 1 };

    let (game_tx, mut network_rx) = mpsc::channel::<Packet>(100);
    let (network_tx, game_rx) = mpsc::channel::<Packet>(100);

    // Sender Task
    let conn_clone = connection.clone();
    tokio::spawn(async move {
        while let Some(pkt) = network_rx.recv().await {
            let data = bincode::serialize(&pkt).unwrap();
            if let Ok(mut stream) = conn_clone.open_uni().await {
                // Write all data to stream
                if let Err(e) = stream.write_all(&data).await {
                    eprintln!("Failed to write to stream: {}", e);
                    continue;
                }
                // Finish the stream (synchronous in this version)
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
                        // Fix: read_to_end expects a size limit, not a buffer
                        // Use a reasonable limit (e.g., 1MB for game packets)
                        match stream.read_to_end(1024 * 1024).await {
                            Ok(buffer) => {
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
                Err(_) => break, // Connection closed
            }
        }
    });

    Ok(NetworkManager {
        tx: game_tx,
        rx: game_rx,
        player_id,
    })
}

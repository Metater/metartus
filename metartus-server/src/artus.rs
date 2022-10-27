use tokio::task::JoinHandle;

use crate::types::ArtusRequest;

pub struct Artus {
    rx: flume::Receiver<ArtusRequest>
}

impl Artus {
    pub fn new(rx: flume::Receiver<ArtusRequest>) -> Self {
        Self {
            rx
        }
    }

    pub fn start(self) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Ok((notifier, command, response)) = self.rx.recv_async().await {
                match command {
                    ArtusCommand::Index => {
                        response.set(ArtusResponse::Index).unwrap();
                    }
                }
                notifier.notify_one();
            }
        })
    }
}

#[derive(Debug)]
pub enum ArtusCommand {
    Index
}

#[derive(Debug)]
pub enum ArtusResponse {
    Index
}
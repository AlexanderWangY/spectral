use std::io::Error;

use tokio::sync::{mpsc, oneshot};

use crate::actor::Actor;

pub enum OverlayMsg {
    Ack,
    Nack,
    GetPrime { reply: oneshot::Sender<u64> },
}

#[derive(Clone)]
pub struct OverlayHandle {
    tx: mpsc::Sender<OverlayMsg>,
}

impl OverlayHandle {
    pub fn new(tx: mpsc::Sender<OverlayMsg>) -> Self {
        Self { tx }
    }

    pub async fn get_prime(&self) -> anyhow::Result<u64> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(OverlayMsg::GetPrime { reply: tx }).await?;
        Ok(rx.await?)
    }
}

struct OverlayActor {}

impl Actor for OverlayActor {
    type Msg = OverlayMsg;
    const NAME: &'static str = "OverlayActor";

    async fn handle(&mut self, msg: Self::Msg) {
        match msg {
            OverlayMsg::Ack => {}
            OverlayMsg::Nack => {}
            OverlayMsg::GetPrime { reply } => {
                // You would actually do something here to get the information
                let _ = reply.send(13);
            }
        }
    }

    async fn on_start(&mut self) {}

    async fn on_stop(&mut self) {}
}

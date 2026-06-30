use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::actor::Actor;

#[derive(Debug)]
pub enum OverlayMsg {}

#[derive(Clone)]
pub struct OverlayHandle {
    tx: mpsc::Sender<OverlayMsg>,
}

impl OverlayHandle {
    pub fn new(tx: mpsc::Sender<OverlayMsg>) -> Self {
        Self { tx }
    }
}

pub struct OverlayActor {}

impl Actor for OverlayActor {
    type Msg = OverlayMsg;
    const NAME: &'static str = "OverlayActor";

    async fn handle(&mut self, msg: Self::Msg) {
        debug!("{}: {:?}", Self::NAME, msg);
    }

    async fn on_start(&mut self) {}

    async fn on_stop(&mut self) {}
}

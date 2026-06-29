use tokio::sync::mpsc;

use crate::actor::run;

pub struct Runtime {}

impl Runtime {
    pub fn start() -> Self {
        // Initialize mspc channels
        let (ovly_tx, mut ovly_rx) = mpsc::channel::<u64>(256);

        // Construct actors

        // Inject channels

        // Kick off run

        Self {}
    }
}

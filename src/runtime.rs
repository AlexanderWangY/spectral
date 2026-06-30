use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    actor::run,
    overlay::{OverlayActor, OverlayHandle},
};

pub struct Runtime {}

impl Runtime {
    pub fn start() -> Self {
        // Cancel token
        let cancel = CancellationToken::new();

        // Initialize mspc channels
        let (tx, rx) = mpsc::channel(256);

        // Construct actors
        let overlay_actor = OverlayActor {};
        let _ = OverlayHandle::new(tx);

        // Inject channels

        // Kick off run
        tokio::spawn(run(overlay_actor, rx, cancel));

        Self {}
    }
}

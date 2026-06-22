use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub trait Actor: Send + 'static {
    type Msg: Send + 'static;
    const NAME: &'static str;

    // Handlers are async, return no value, and must be movable across threads.
    fn handle(&mut self, msg: Self::Msg) -> impl Future<Output = ()> + Send;

    fn on_start(&mut self) -> impl Future<Output = ()> + Send {
        async {}
    }
    fn on_stop(&mut self) -> impl Future<Output = ()> + Send {
        async {}
    }
}

pub async fn run<A: Actor>(
    mut actor: A,
    mut inbox: mpsc::Receiver<A::Msg>,
    cancel: CancellationToken,
) {
    actor.on_start().await;
    loop {
        tokio::select! {
            maybe = inbox.recv() => match maybe {
                Some(msg) => actor.handle(msg).await,
                None => break
            },
            _ = cancel.cancelled() => break,
        }
    }
    actor.on_stop().await;
}

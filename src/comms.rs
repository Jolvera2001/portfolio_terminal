use futures::future::BoxFuture;
use tokio::sync::mpsc;

use crate::{portfolio::GlobalMsg, views::guide::GuideMsg};

pub enum Msg {
    Global(GlobalMsg),
    Guide(GuideMsg),
}

pub type CommandFuture<Msg> = BoxFuture<'static, Msg>;
pub struct Command<Msg> {
    futures: Vec<CommandFuture<Msg>>,
}

impl<Msg: Send + 'static> Command<Msg> {
    pub fn none() -> Self {
        Self { futures: vec![] }
    }

    pub fn perform<F>(future: F) -> Self
    where
        F: std::future::Future<Output = Msg> + Send + 'static,
    {
        Self {
            futures: vec![Box::pin(future)],
        }
    }

    pub fn batch(commands: Vec<Self>) -> Self {
        Self {
            futures: commands.into_iter().flat_map(|cmd| cmd.futures).collect(),
        }
    }

    pub fn execute(self, tx: mpsc::UnboundedSender<Msg>) {
        for fut in self.futures {
            let tx = tx.clone();
            tokio::spawn(async move {
                let msg = fut.await;
                let _ = tx.send(msg);
            });
        }
    }
}

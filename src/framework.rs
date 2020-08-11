use serenity::{
    framework::Framework,
    model::channel::Message,
    client::Context
};
use threadpool::ThreadPool;
use std::sync::mpsc::Sender;

pub struct ShillFramework {
    pub channel_tx: Sender<u64>
}

impl Framework for ShillFramework {
    fn dispatch(&mut self, _: Context, _: Message, pool: &ThreadPool) {
        let tx = self.channel_tx.clone();
        pool.execute(move || {
            tx.send(0).unwrap();
        });
    }
}
use futures::{
    sync::mpsc::UnboundedReceiver,
    Poll,
    Stream,
};
use serenity::gateway::Shard;
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// The shard spawner is a stream of instantiated shards.
///
/// This is returned by [`spawn`]. A shard is sent over the stream once it
/// becomes ready.
///
/// Once all shards have been spawned according to the [`ShardingStrategy`]
/// requested, the stream will permanently end.
///
/// # Examples
///
/// Refer to the [`spawn` examples] for more information.
///
/// [`ShardingStrategy`]:
/// [`spawn` examples]: fn.spawn.html#examples
/// [`spawn`]: fn.spawn.html
pub struct ShardSpawner {
    inner: UnboundedReceiver<Shard>,
    __nonexhaustive: (),
}

impl ShardSpawner {
    pub(crate) fn new(inner: UnboundedReceiver<Shard>) -> Self {
        Self {
            __nonexhaustive: (),
            inner,
        }
    }
}

impl Debug for ShardSpawner {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_struct("ShardSpawner")
            .field("inner", &"Unbounded Receiver of T Shard")
            .finish()
    }
}

impl Stream for ShardSpawner {
    type Item = Shard;
    type Error = ();

    /// Polls the inner receiver.
    ///
    /// # Logs
    ///
    /// [TRACE] That the inner receiver is being polled.
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        trace!("Polling inner receiver");

        self.inner.poll()
    }
}

use futures::{
    future::{self, Loop},
    sync::mpsc::{self, UnboundedSender},
    Future,
    Stream,
};
use serenity::gateway::Shard;
use std::time::{Duration, Instant};
use tokio::{
    executor::{DefaultExecutor, Executor},
    timer::Delay,
};
use {Error, ShardSpawner, SharderOptions};

struct LoopState {
    end: u64,
    id: u64,
    total: u64,
    tx: UnboundedSender<Shard>,
}

impl LoopState {
    fn new(
        start: u64,
        end: u64,
        total: u64,
        tx: UnboundedSender<Shard>,
    ) -> Self {
        Self {
            id: start,
            end,
            total,
            tx,
        }
    }
}

/// Spawns a new [`ShardSpawner`], which is a stream of shards as they spawn
/// and become "ready".
///
/// These are spawned in a queue according to the value of
/// [`SharderOptions::delay`].
///
/// # Examples
///
/// Start a bot by taking an environment variable containing the bot token,
/// utilizing the autoshard strategy, and then printing the received
/// `MessageCreate`s of all the shards via the `log` crate:
///
/// ```rust,no_run
/// # extern crate futures;
/// # extern crate serenity;
/// # extern crate serenity_sharder;
/// # extern crate tokio;
/// #
/// # #[macro_use]
/// # extern crate log;
/// #
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<Error>> {
/// #
/// use futures::{Future, Stream, future};
/// use serenity::{
///     model::event::{Event, GatewayEvent},
///     gateway::Shard,
/// };
/// use serenity_sharder::prelude::*;
/// use std::env;
/// use tokio;
///
/// // Retrieve the discord bot token from the environment.
/// let token = env::var("DISCORD_TOKEN")?;
///
/// // Create a new set of options, using the autoshard strategy and delaying
/// // shard boots by 10 seconds.
/// let mut options = SharderOptions::new(token);
/// options.delay(10).strategy(ShardingStrategy::Autoshard);
///
/// // Create a stream of shards, log to INFO when they're booted, and then log
/// // in DEBUG on all `MessageCreate`s received.
/// let future = serenity_sharder::spawn(options)?.for_each(|shard| {
///     info!("Booted shard {:?}", shard.shard_info());
///
///     // Loop over WebSocket messages received from Discord.
///     shard.messages().into_future().then(|data| -> Box<Future<Item = (), Error = SharderError>> {
///         let (msg, stream) = match data {
///             Ok(data) => data,
///             Err(why) => return Box::new(future::err(SharderError::from(why))),
///         };
///         let msg = match msg {
///             Some(msg) => msg,
///             None => panic!(),
///         };
///
///         // Have the shard parse the message into a deserialized event.
///         let event = match shard.parse(&msg) {
///             Ok(event) => event,
///             Err((why, _)) => return Box::new(future::err(SharderError::from(why))),
///         };
///
///         // Have the shard process, the WebSocket event, in case it needs to
///         // mutate its state, send a packet, etc.
///         //
///         // This can give back a future in the event something needs to be
///         // done, such as waiting for a reconnection.
///         if let Some(future) = shard.process(&event) {
///             Box::new(future
///                 .map(move |_| ())
///                 .map_err(SharderError::from))
///         } else {
///             Box::new(future::ok(())
///                 .map(move |_| ())
///                 .map_err(SharderError::from))
///         }
///     }).map_err(|_| ())
/// });
///
/// // Finally, run the sharder on the tokio runtime.
/// tokio::run(future);
/// #
/// #     Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns [`Error::TokioExecutor`] when there is an issue spawning a future on
/// the default executor.
///
/// # Logs
///
/// [DEBUG] What the values of the sharding strategy are.
/// [DEBUG] When attempting to spawn a shard, indicating what the ID and total
/// is.
/// [DEBUG] The ID of the booted shard, and what the delay until the next spawn
/// is.
/// [DEBUG] When finished sharding and the loop is breaking.
/// [INFO] When the sharding strategy has been completed.
///
/// [`Error::TokioExecutor`]: enum.Error.html#variant.TokioExecutor
/// [`ShardSpawner`]: struct.ShardSpawner.html
/// [`SharderOptions::delay`]: struct.SharderOptions.html#structfield.delay
pub fn spawn(
    options: SharderOptions,
) -> Result<impl Stream<Item = Shard, Error = ()>, Error> {
    let values = options.strategy.values().unwrap_or((0, 1, 0));
    debug!("Using strategy values of: {:?}", values);
    let (start, end, total) = values;

    let (tx, rx) = mpsc::unbounded();
    let state = LoopState::new(start, end, total, tx);
    let delay = options.delay;

    let sharder = future::loop_fn(state, move |state| {
        debug!("Attempting to boot shard {} of {}", state.id, state.end);

        Shard::new(options.token.to_owned(), [state.id, state.total])
            .from_err::<Error>()
            .map(move |shard| {
                state.tx.unbounded_send(shard).expect("Error sending shard");

                state
            }).and_then(move |state| {
                let until = Instant::now() + Duration::from_secs(delay);
                debug!("Booted shard {}, delaying until {:?}", state.id, until);

                Delay::new(until).map(|_| state).from_err()
            }).and_then(|mut state| {
                if state.id == state.end {
                    debug!("Finished sharding, breaking loop...");

                    Ok(Loop::Break(state))
                } else {
                    state.id += 1;

                    Ok(Loop::Continue(state))
                }
            })
    }).map(|_| {
        info!("Completed shard strategy");
    }).map_err(|_| ());

    DefaultExecutor::current().spawn(Box::new(sharder))?;

    Ok(ShardSpawner::new(rx))
}

//! # serenity-sharder
//!
//! Sharder is a library used to create a stream of asynchronously booted and
//! operational Discord shards, especially executed by the `tokio` runtime.
//! Sharder is backed by the [serenity] library.
//!
//! Shards can be plugged into a pre-made adapter to be transported to a message
//! broker of choice.
//!
//! View the [adapters repository] for a list of available adapters.
//!
//! # Examples
//!
//! Start a bot by taking an environment variable containing the bot token,
//! utilizing the autoshard strategy, delaying boots by 10 seconds, and then
//! printing the ID of every shard spawned:
//!
//! ```rust,no_run
//! # extern crate futures;
//! # extern crate serenity_sharder;
//! # extern crate tokio;
//! #
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<Error>> {
//! #
//! use futures::Stream;
//! use serenity_sharder::{self, SharderOptions, ShardingStrategy};
//! use std::env;
//!
//! let token = env::var("DISCORD_TOKEN")?;
//! let mut options = SharderOptions::new(token);
//! options.delay(10).strategy(ShardingStrategy::Autoshard);
//!
//! let future = serenity_sharder::spawn(options)?.for_each(|shard| {
//!     println!("Shard spawned! {:?}", shard.shard_info());
//!
//!     Ok(())
//! });
//!
//! tokio::run(future);
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! [adapters repository]: https://github.com/serenity-rs/adapters
//! [serenity]: https://github.com/serenity-rs/serenity
#![deny(missing_docs, unknown_lints)]

extern crate futures;
extern crate serenity;
extern crate tokio;
extern crate tungstenite;

#[macro_use]
extern crate log;

pub mod prelude;

mod error;
mod options;
mod spawn;
mod spawner;
mod strategy;

pub use self::{
    error::Error,
    options::SharderOptions,
    spawn::spawn,
    spawner::ShardSpawner,
    strategy::ShardingStrategy,
};

use futures::Future;
use serenity::{
    gateway::Shard,
    model::event::GatewayEvent,
};

/// The return type of a closure after having a serenity shard process an event.
///
/// This is not necessarily _always_ the return type, but often it will be, and
/// thus is exported for ease-of-use.
///
/// This is also re-exported from the [`prelude`].
///
/// [`prelude`]: ./prelude/
pub type MessageProcessed =
    Box<Future<Item = (Shard, GatewayEvent), Error = Error>>;

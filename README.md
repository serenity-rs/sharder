# serenity-sharder

Sharder is a library used to create a stream of asynchronously booted and
operational Discord shards, especially executed by the `tokio` runtime.
Sharder is backed by the [serenity] library.

Shards can be plugged into a pre-made adapter to be transported to a message
broker of choice.

View the [adapters repository] for a list of available adapters.

### Why?

The sharder is intended to essentially be a WebSocket proxy, separate from the
remainder of the bot's infrastructure. In this sense, the rest of the bot's
components can be restarted - either via hot-reloading or an entire
restart of the processes - and will not affect uptime.

TODO: A writeup on this will be posted soon.

### Examples

Start a bot by taking an environment variable containing the bot token,
utilizing the autoshard strategy, delaying boots by 10 seconds, and then
printing the ID of every shard spawned:

```rust
extern crate futures;
extern crate serenity_sharder;
extern crate tokio;

use futures::Stream;
use serenity_sharder::{self, SharderOptions, ShardingStrategy};
use std::{
    error::Error,
    env,
};

fn main() -> Result<(), Box<Error>> {
    let token = env::var("DISCORD_TOKEN")?;
    let mut options = SharderOptions::new(token);
    options.delay(10).strategy(ShardingStrategy::Autoshard);

    let future = serenity_sharder::spawn(options)?.for_each(|shard| {
        println!("Shard spawned! {:?}", shard.shard_info());

        Ok(())
    });

    tokio::run(future);

    Ok(())
}
```

[adapters repository]: https://github.com/serenity-rs/adapters
[serenity]: https://github.com/serenity-rs/serenity

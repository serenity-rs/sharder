use ShardingStrategy;

/// Options to use when creating a new sharder.
///
/// Only the [`token`] is required to be specified by the user. Refer to each
/// field for their default value.
///
/// This is used when calling [`spawn`].
///
/// [`spawn`]: fn.spawn.html
/// [`token`]: #structfield.token
#[derive(Clone, Debug)]
pub struct SharderOptions {
    /// The number of seconds to wait between each shard start.
    ///
    /// This must be at least 5.
    ///
    /// Defaults to 6.
    pub delay: u64,
    /// The strategy to use for sharding.
    ///
    /// Defaults to [`ShardingStrategy::Autoshard`].
    pub strategy: ShardingStrategy,
    /// The bot's token.
    pub token: String,
    __nonexhaustive: (),
}

impl SharderOptions {
    /// Creates a new set of options for spawning shards.
    ///
    /// # Examples
    ///
    /// Creating new options and setting the [`delay`] to 10 seconds:
    ///
    /// ```rust,no_run
    /// # extern crate futures;
    /// # extern crate serenity_sharder;
    /// #
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<Error>> {
    /// #
    /// use serenity_sharder::{self, SharderOptions};
    /// use std::env;
    ///
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let mut options = SharderOptions::new(token);
    /// options.delay(10);
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new(token: impl ToString) -> Self {
        Self::_new(token.to_string())
    }

    fn _new(token: String) -> Self {
        Self {
            delay: 6,
            strategy: ShardingStrategy::Autoshard,
            __nonexhaustive: (),
            token,
        }
    }

    /// Sets the delay between shard starts.
    ///
    /// Refer to [`delay`] for more information.
    ///
    /// # Examples
    ///
    /// Set a delay of 10 seconds:
    ///
    /// ```rust,no_run
    /// # extern crate serenity_sharder;
    /// #
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<Error>> {
    /// #
    /// use serenity_sharder::SharderOptions;
    /// use std::env;
    ///
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let mut options = SharderOptions::new(token);
    /// options.delay(10);
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// # Logs
    ///
    /// [WARN] When the sharding delay was below the minimum and was
    /// automatically increased.
    ///
    /// [`delay`]: #structfield.delay
    pub fn delay(&mut self, delay: u64) -> &mut Self {
        self.delay = if delay >= 5 {
            delay
        } else {
            warn!("Increased sharding delay from {} to 5", delay);

            5
        };

        self
    }

    /// Sets the sharding strategy to use for starting shards.
    ///
    /// Refer to [`strategy`] for more information.
    ///
    /// # Examples
    ///
    /// Set the sharding strategy to use a range of shards 0 to 19 of 40:
    ///
    /// ```rust,no_run
    /// # extern crate serenity_sharder;
    /// #
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<Error>> {
    /// #
    /// use serenity_sharder::{SharderOptions, ShardingStrategy};
    /// use std::env;
    ///
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let mut options = SharderOptions::new(token);
    /// options.strategy(ShardingStrategy::Range(0, 19, 40));
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`strategy`]: #structfield.strategy
    pub fn strategy(&mut self, strategy: ShardingStrategy) -> &mut Self {
        self.strategy = strategy;

        self
    }

    /// Sets the bot's token.
    ///
    /// Refer to [`token`] for more information.
    ///
    /// # Examples
    ///
    /// Set the token to a new token after-the-fact:
    ///
    /// ```rust,no_run
    /// # extern crate serenity_sharder;
    /// #
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<Error>> {
    /// #
    /// use serenity_sharder::SharderOptions;
    /// use std::env;
    ///
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let mut options = SharderOptions::new(token);
    ///
    /// // work here
    /// # let use_alternative_token = true;
    /// if use_alternative_token {
    ///     options.token(env::var("ALTERNATE_TOKEN")?);
    /// }
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`token`]: #structfield.token
    #[inline]
    pub fn token(&mut self, token: impl ToString) -> &mut Self {
        self._token(token.to_string())
    }

    fn _token(&mut self, token: String) -> &mut Self {
        self.token = token;

        self
    }
}

#[cfg(test)]
mod tests {
    use {SharderOptions, ShardingStrategy};

    #[test]
    fn test_fields() {
        let mut options = SharderOptions::new("aaa");
        options.delay(7).strategy(ShardingStrategy::Autoshard);

        assert_eq!(options.delay, 7);
        assert_eq!(options.strategy, ShardingStrategy::Autoshard);
        assert_eq!(options.token, "aaa");
    }

    #[test]
    fn test_generics() {
        fn decorator(foo: impl ToString) -> impl ToString {
            foo
        }

        let mut options = SharderOptions::new(decorator("a"));

        assert_eq!(options.token, "a");

        options.token(decorator("b"));
        assert_eq!(options.token, "b");
    }

    #[test]
    fn test_minimum_delay() {
        let mut options = SharderOptions::new("0");
        options.delay(6);
        assert_eq!(options.delay, 6);

        // Assert that 4 becomes 5
        options.delay(4);
        assert_eq!(options.delay, 5);
    }
}

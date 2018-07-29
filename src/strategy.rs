/// The strategy to use for starting shards in a sharder context.
///
/// Refer to each variant for more information.
///
/// Defaults to [`ShardingStrategy::Autoshard`].
///
/// [`ShardingStrategy::Autoshard`]: #variant.Autoshard
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum ShardingStrategy {
    /// Specify that a default number of shards will be used. This is
    /// proportionate to the size of your bot in guilds.
    ///
    /// Typically you should use this.
    Autoshard,
    /// Specify that a range of shards will be started.
    ///
    /// This is useful when you have a large bot and want to split the load
    /// across different machines.
    ///
    /// This is used in the [`ShardingOptions`].
    ///
    /// # Examples
    ///
    /// Start shards 0 through 19 of 40 total:
    ///
    /// ```rust,no_run
    /// use serenity_sharder::ShardingStrategy;
    ///
    /// let strategy = ShardingStrategy::range(0, 19, 40);
    /// ```
    ///
    /// [`ShardingOptions`]: struct.ShardingOptions.html
    Range(u64, u64, u64),
    #[doc(hidden)]
    Nonexhaustive,
}

impl ShardingStrategy {
    /// Returns the autoshard strategy.
    ///
    /// Refer to [`ShardingStrategy::Autoshard`] for more information, as this
    /// is just a wrapper over it.
    ///
    /// [`ShardingStrategy::Autoshard`]: #variant.Autoshard
    pub fn auto() -> Self {
        ShardingStrategy::Autoshard
    }

    /// Returns a range strategy.
    ///
    /// Refer to [`ShardingStrategy::Range`] for more information, as this is
    /// just a wrapper over it.
    ///
    /// [`ShardingStrategy::Range`]: #variant.Range
    pub fn range(start: u64, amount: u64, total: u64) -> Self {
        ShardingStrategy::Range(start, amount, total)
    }

    /// Returns the values of the shards that should be booted.
    ///
    /// The three values in the tuple are, in order:
    ///
    /// - the ID of the first shard to start
    /// - the number of shards to start, incrementing the ID for each
    /// - the total number of shards utilized by the bot across all instances
    ///
    /// # Examples
    ///
    /// Create a sharding strategy booting shards 0 through 4, inclusively, out
    /// of 10 total:
    ///
    /// ```rust
    /// use serenity_sharder::ShardingStrategy;
    ///
    /// let strategy = ShardingStrategy::Range(0, 5, 10);
    /// assert_eq!(strategy.values(), Some((0, 5, 10)));
    /// ```
    ///
    /// Assert that [`ShardingStrategy::Autoshard`] has no defined range, as
    /// it's dynamic:
    ///
    /// ```rust
    /// use serenity_sharder::ShardingStrategy;
    ///
    /// let strategy = ShardingStrategy::Autoshard;
    /// assert!(strategy.values().is_none());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics when this is called on an undocumented variant.
    ///
    /// [`ShardingStrategy::Autoshard`]: #variant.Autoshard
    pub fn values(&self) -> Option<(u64, u64, u64)> {
        use ShardingStrategy::*;

        match *self {
            Range(start, end, total) => Some((start, end, total)),
            Autoshard => None,
            Nonexhaustive => unreachable!("Don't specify this"),
        }
    }
}

impl Default for ShardingStrategy {
    fn default() -> Self {
        ShardingStrategy::Autoshard
    }
}

#[cfg(test)]
mod tests {
    use super::ShardingStrategy;

    #[test]
    fn test_auto() {
        assert_eq!(ShardingStrategy::Autoshard, ShardingStrategy::auto());
    }

    #[test]
    fn test_default() {
        assert_eq!(ShardingStrategy::default(), ShardingStrategy::Autoshard);
    }

    #[test]
    fn test_range() {
        let (lhs, rhs) = (
            ShardingStrategy::Range(0, 4, 10),
            ShardingStrategy::range(0, 4, 10),
        );

        assert_eq!(lhs, rhs);
    }

    #[should_panic]
    #[test]
    fn test_strategy_panic() {
        let _ = ShardingStrategy::Nonexhaustive.values();
    }
}

//! Commonly used types re-exported all in one for ease-of-use.
//!
//! # Examples
//!
//! Import all of the prelude:
//!
//! ```rust,no_run
//! use serenity_sharder::prelude::*;
//! ```

pub use {
    Error as SharderError,
    MessageProcessed,
    SharderOptions,
    ShardingStrategy,
};

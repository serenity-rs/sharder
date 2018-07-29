use serenity::Error as SerenityError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};
use tokio::{
    executor::SpawnError as ExecutorSpawnError,
    timer::Error as TimerError,
};
use tungstenite::Error as TungsteniteError;

/// Enum wrapping all of the library's potential errors.
#[derive(Debug)]
pub enum Error {
    /// An error from the `serenity` crate, likely when booting a shard.
    Serenity(SerenityError),
    /// An error from `tokio::timer`, likely from an issue creating the timer.
    Timer(TimerError),
    /// An error from `tokio::executor` when spawning on the default executor.
    TokioExecutor(ExecutorSpawnError),
    /// An error from the `tungstenite` crate.
    Tungstenite(TungsteniteError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            Serenity(ref inner) => inner.description(),
            Timer(ref inner) => inner.description(),
            TokioExecutor(_) => {
                "An error occurred while spawning on the executor"
            },
            Tungstenite(ref inner) => inner.description(),
        }
    }
}

impl From<SerenityError> for Error {
    fn from(err: SerenityError) -> Self {
        Error::Serenity(err)
    }
}

impl From<TimerError> for Error {
    fn from(err: TimerError) -> Self {
        Error::Timer(err)
    }
}

impl From<ExecutorSpawnError> for Error {
    fn from(err: ExecutorSpawnError) -> Self {
        Error::TokioExecutor(err)
    }
}

impl From<TungsteniteError> for Error {
    fn from(err: TungsteniteError) -> Self {
        Error::Tungstenite(err)
    }
}

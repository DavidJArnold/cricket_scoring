pub mod error;
pub mod scoring;

#[cfg(feature = "cricsheet")]
pub mod cricsheet;

// Re-export commonly used types at the crate root for convenience
pub use scoring::{
    BallEvents, BallOutcome, CurrentScore, Innings, Match, MatchResult, MatchStatus, MatchType,
    Player, Team, Wicket, WinMargin,
};

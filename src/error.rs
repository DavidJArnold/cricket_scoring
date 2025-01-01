use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum BallString {
    #[error("Ball string can't be empty")]
    EmptyBallString,
    #[error("Ball string can't contain {0}")]
    InvalidBallStringCharacter(char),
    #[error(
        "If byes are indicated, the number of runs to be added to the total must be indicated"
    )]
    InvalidByeCharacter,
    #[error("Only zero or one of F/S, or L/B can appear")]
    InvalidBallDescription,
}

#[derive(Error, Debug, Clone)]
pub enum BallOutcomeValidation {
    #[error("Incompatible double outcomes {0} and {1} given.")]
    DoubleOutcome(String, String),
}

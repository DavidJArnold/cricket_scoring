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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball_string_empty_error() {
        let error = BallString::EmptyBallString;
        assert_eq!(error.to_string(), "Ball string can't be empty");
    }

    #[test]
    fn test_ball_string_invalid_character_error() {
        let error = BallString::InvalidBallStringCharacter('Z');
        assert_eq!(error.to_string(), "Ball string can't contain Z");
    }

    #[test]
    fn test_ball_string_invalid_bye_error() {
        let error = BallString::InvalidByeCharacter;
        assert_eq!(
            error.to_string(),
            "If byes are indicated, the number of runs to be added to the total must be indicated"
        );
    }

    #[test]
    fn test_ball_string_invalid_description_error() {
        let error = BallString::InvalidBallDescription;
        assert_eq!(
            error.to_string(),
            "Only zero or one of F/S, or L/B can appear"
        );
    }

    #[test]
    fn test_ball_string_clone() {
        let original = BallString::EmptyBallString;
        let cloned = original.clone();
        assert_eq!(original.to_string(), cloned.to_string());
    }

    #[test]
    fn test_ball_string_debug() {
        let error = BallString::InvalidBallStringCharacter('X');
        let debug_string = format!("{:?}", error);
        assert!(debug_string.contains("InvalidBallStringCharacter"));
        assert!(debug_string.contains("'X'"));
    }

    #[test]
    fn test_ball_outcome_validation_double_outcome() {
        let error = BallOutcomeValidation::DoubleOutcome("Four".to_string(), "Six".to_string());
        assert_eq!(
            error.to_string(),
            "Incompatible double outcomes Four and Six given."
        );
    }

    #[test]
    fn test_ball_outcome_validation_clone() {
        let original =
            BallOutcomeValidation::DoubleOutcome("Bye".to_string(), "Leg Bye".to_string());
        let cloned = original.clone();
        assert_eq!(original.to_string(), cloned.to_string());
    }

    #[test]
    fn test_ball_outcome_validation_debug() {
        let error = BallOutcomeValidation::DoubleOutcome("Test1".to_string(), "Test2".to_string());
        let debug_string = format!("{:?}", error);
        assert!(debug_string.contains("DoubleOutcome"));
        assert!(debug_string.contains("Test1"));
        assert!(debug_string.contains("Test2"));
    }

    #[test]
    fn test_different_ball_string_errors() {
        let errors = vec![
            BallString::EmptyBallString,
            BallString::InvalidBallStringCharacter('Y'),
            BallString::InvalidByeCharacter,
            BallString::InvalidBallDescription,
        ];

        // Each error should have a different message
        let messages: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        for (i, msg1) in messages.iter().enumerate() {
            for (j, msg2) in messages.iter().enumerate() {
                if i != j {
                    assert_ne!(msg1, msg2, "Error messages should be different");
                }
            }
        }
    }

    #[test]
    fn test_ball_string_error_trait() {
        let error = BallString::EmptyBallString;
        // Test that it implements the Error trait
        let _error_trait: &dyn std::error::Error = &error;
    }

    #[test]
    fn test_ball_outcome_validation_error_trait() {
        let error = BallOutcomeValidation::DoubleOutcome("A".to_string(), "B".to_string());
        // Test that it implements the Error trait
        let _error_trait: &dyn std::error::Error = &error;
    }
}

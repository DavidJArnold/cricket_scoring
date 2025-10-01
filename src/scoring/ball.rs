use super::player::Player;
use serde::{Deserialize, Serialize};

use crate::error::BallOutcomeValidation;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Wicket {
    pub player_out: String,
    pub kind: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum BallEvents {
    Bye(i32),
    LegBye(i32),
    NoBall(i32),
    Wicket(Vec<Wicket>),
    Wide(i32),
    Penalty(i32),
    Four,
    Six,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BallOutcome {
    pub runs: i32,
    pub wicket: Option<Vec<Wicket>>,
    pub no_ball: Option<i32>,
    pub wide: Option<i32>,
    pub byes: Option<i32>,
    pub leg_byes: Option<i32>,
    pub free_hit: bool,
    pub four: bool,
    pub six: bool,
    pub on_strike: Player,
    pub off_strike: Player,
    pub bowler: Player,
    pub penalty: Option<i32>,
}

impl BallOutcome {
    #[must_use]
    pub fn new(
        runs: i32,
        ball_events: Vec<BallEvents>,
        on_strike: Player,
        off_strike: Player,
        bowler: Player,
    ) -> BallOutcome {
        let mut outcome = BallOutcome {
            runs,
            on_strike,
            off_strike,
            bowler,
            ..BallOutcome::default()
        };
        for event in ball_events {
            match event {
                BallEvents::Bye(x) => outcome.byes = Some(x),
                BallEvents::LegBye(x) => outcome.leg_byes = Some(x),
                BallEvents::NoBall(x) => outcome.no_ball = Some(x),
                BallEvents::Wicket(x) => outcome.wicket = Some(x),
                BallEvents::Wide(x) => outcome.wide = Some(x),
                BallEvents::Four => outcome.four = true,
                BallEvents::Six => outcome.six = true,
                BallEvents::Penalty(x) => outcome.penalty = Some(x),
            }
        }
        outcome
    }

    /// # Errors
    ///
    /// Will return an error based on the problem encountered during validation
    pub fn validate(&self) -> Result<(), BallOutcomeValidation> {
        if self.four && self.six {
            return Err(BallOutcomeValidation::DoubleOutcome(
                "Four".to_string(),
                "Six".to_string(),
            ));
        }
        if self.byes.is_some() && self.leg_byes.is_some() {
            return Err(BallOutcomeValidation::DoubleOutcome(
                "Bye".to_string(),
                "Leg Bye".to_string(),
            ));
        }
        // if self.four && self.runs != 4 {
        //     return false
        // }
        // if self.six && self.runs != 6 {
        //     return false
        // }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_player(name: &str) -> Player {
        Player::new(name.to_string())
    }

    #[test]
    fn test_wicket_creation() {
        let wicket = Wicket {
            player_out: "John Doe".to_string(),
            kind: "bowled".to_string(),
        };
        assert_eq!(wicket.player_out, "John Doe");
        assert_eq!(wicket.kind, "bowled");
    }

    #[test]
    fn test_wicket_clone() {
        let wicket = Wicket {
            player_out: "Jane Smith".to_string(),
            kind: "caught".to_string(),
        };
        let cloned = wicket.clone();
        assert_eq!(wicket, cloned);
    }

    #[test]
    fn test_ball_events_equality() {
        assert_eq!(BallEvents::Bye(2), BallEvents::Bye(2));
        assert_eq!(BallEvents::LegBye(1), BallEvents::LegBye(1));
        assert_eq!(BallEvents::NoBall(1), BallEvents::NoBall(1));
        assert_eq!(BallEvents::Wide(1), BallEvents::Wide(1));
        assert_eq!(BallEvents::Penalty(5), BallEvents::Penalty(5));
        assert_eq!(BallEvents::Four, BallEvents::Four);
        assert_eq!(BallEvents::Six, BallEvents::Six);

        let wicket1 = vec![Wicket {
            player_out: "Player1".to_string(),
            kind: "bowled".to_string(),
        }];
        let wicket2 = vec![Wicket {
            player_out: "Player1".to_string(),
            kind: "bowled".to_string(),
        }];
        assert_eq!(BallEvents::Wicket(wicket1), BallEvents::Wicket(wicket2));
    }

    #[test]
    fn test_ball_outcome_default() {
        let outcome = BallOutcome::default();
        assert_eq!(outcome.runs, 0);
        assert!(outcome.wicket.is_none());
        assert!(outcome.no_ball.is_none());
        assert!(outcome.wide.is_none());
        assert!(outcome.byes.is_none());
        assert!(outcome.leg_byes.is_none());
        assert!(!outcome.free_hit);
        assert!(!outcome.four);
        assert!(!outcome.six);
        assert!(outcome.penalty.is_none());
    }

    #[test]
    fn test_ball_outcome_new_simple() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let outcome = BallOutcome::new(
            1,
            vec![],
            on_strike.clone(),
            off_strike.clone(),
            bowler.clone(),
        );

        assert_eq!(outcome.runs, 1);
        assert_eq!(outcome.on_strike.name, "Batsman1");
        assert_eq!(outcome.off_strike.name, "Batsman2");
        assert_eq!(outcome.bowler.name, "Bowler");
        assert!(outcome.wicket.is_none());
        assert!(!outcome.four);
        assert!(!outcome.six);
    }

    #[test]
    fn test_ball_outcome_new_with_four() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let outcome = BallOutcome::new(4, vec![BallEvents::Four], on_strike, off_strike, bowler);

        assert_eq!(outcome.runs, 4);
        assert!(outcome.four);
        assert!(!outcome.six);
    }

    #[test]
    fn test_ball_outcome_new_with_six() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let outcome = BallOutcome::new(6, vec![BallEvents::Six], on_strike, off_strike, bowler);

        assert_eq!(outcome.runs, 6);
        assert!(!outcome.four);
        assert!(outcome.six);
    }

    #[test]
    fn test_ball_outcome_new_with_wicket() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");
        let wicket = vec![Wicket {
            player_out: "Batsman1".to_string(),
            kind: "bowled".to_string(),
        }];

        let outcome = BallOutcome::new(
            0,
            vec![BallEvents::Wicket(wicket.clone())],
            on_strike,
            off_strike,
            bowler,
        );

        assert_eq!(outcome.runs, 0);
        assert_eq!(outcome.wicket, Some(wicket));
    }

    #[test]
    fn test_ball_outcome_new_with_wide() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let outcome = BallOutcome::new(1, vec![BallEvents::Wide(1)], on_strike, off_strike, bowler);

        assert_eq!(outcome.runs, 1);
        assert_eq!(outcome.wide, Some(1));
    }

    #[test]
    fn test_ball_outcome_new_with_no_ball() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let outcome = BallOutcome::new(
            1,
            vec![BallEvents::NoBall(1)],
            on_strike,
            off_strike,
            bowler,
        );

        assert_eq!(outcome.runs, 1);
        assert_eq!(outcome.no_ball, Some(1));
    }

    #[test]
    fn test_ball_outcome_new_with_bye() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let outcome = BallOutcome::new(2, vec![BallEvents::Bye(2)], on_strike, off_strike, bowler);

        assert_eq!(outcome.runs, 2);
        assert_eq!(outcome.byes, Some(2));
    }

    #[test]
    fn test_ball_outcome_new_with_leg_bye() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let outcome = BallOutcome::new(
            1,
            vec![BallEvents::LegBye(1)],
            on_strike,
            off_strike,
            bowler,
        );

        assert_eq!(outcome.runs, 1);
        assert_eq!(outcome.leg_byes, Some(1));
    }

    #[test]
    fn test_ball_outcome_new_with_penalty() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let outcome = BallOutcome::new(
            5,
            vec![BallEvents::Penalty(5)],
            on_strike,
            off_strike,
            bowler,
        );

        assert_eq!(outcome.runs, 5);
        assert_eq!(outcome.penalty, Some(5));
    }

    #[test]
    fn test_ball_outcome_new_with_multiple_events() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let outcome = BallOutcome::new(
            5,
            vec![BallEvents::NoBall(1), BallEvents::Four],
            on_strike,
            off_strike,
            bowler,
        );

        assert_eq!(outcome.runs, 5);
        assert_eq!(outcome.no_ball, Some(1));
        assert!(outcome.four);
    }

    #[test]
    fn test_validate_valid_outcome() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let outcome = BallOutcome::new(1, vec![], on_strike, off_strike, bowler);
        assert!(outcome.validate().is_ok());
    }

    #[test]
    fn test_validate_four_and_six_error() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let mut outcome =
            BallOutcome::new(4, vec![BallEvents::Four], on_strike, off_strike, bowler);
        outcome.six = true; // Manually set both four and six

        let result = outcome.validate();
        assert!(result.is_err());
        match result.unwrap_err() {
            BallOutcomeValidation::DoubleOutcome(event1, event2) => {
                assert_eq!(event1, "Four");
                assert_eq!(event2, "Six");
            }
        }
    }

    #[test]
    fn test_validate_bye_and_leg_bye_error() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");

        let mut outcome =
            BallOutcome::new(2, vec![BallEvents::Bye(2)], on_strike, off_strike, bowler);
        outcome.leg_byes = Some(1); // Manually set both byes and leg byes

        let result = outcome.validate();
        assert!(result.is_err());
        match result.unwrap_err() {
            BallOutcomeValidation::DoubleOutcome(event1, event2) => {
                assert_eq!(event1, "Bye");
                assert_eq!(event2, "Leg Bye");
            }
        }
    }

    #[test]
    fn test_validate_complex_valid_outcome() {
        let on_strike = create_test_player("Batsman1");
        let off_strike = create_test_player("Batsman2");
        let bowler = create_test_player("Bowler");
        let wicket = vec![Wicket {
            player_out: "Batsman1".to_string(),
            kind: "caught".to_string(),
        }];

        let outcome = BallOutcome::new(
            1,
            vec![BallEvents::Wicket(wicket), BallEvents::NoBall(1)],
            on_strike,
            off_strike,
            bowler,
        );

        assert!(outcome.validate().is_ok());
    }
}

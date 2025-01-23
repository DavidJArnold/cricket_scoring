use super::player::Player;

use crate::error::BallOutcomeValidation;

#[derive(PartialEq)]
pub enum BallEvents {
    Bye,
    LegBye,
    NoBall,
    Wicket,
    Wide,
    Four,
    Six,
}

#[derive(Default)]
pub struct BallOutcome {
    pub runs: i32,
    pub wicket: bool,
    pub no_ball: bool,
    pub wide: bool,
    pub byes: bool,
    pub leg_byes: bool,
    pub free_hit: bool,
    pub four: bool,
    pub six: bool,
    pub on_strike: Player,
    pub off_strike: Player,
}

impl BallOutcome {
    #[must_use]
    pub fn new(runs: i32, ball_events: Vec<BallEvents>) -> BallOutcome {
        let mut outcome = BallOutcome {
            runs,
            ..BallOutcome::default()
        };
        for event in ball_events {
            match event {
                BallEvents::Bye => outcome.byes = true,
                BallEvents::LegBye => outcome.leg_byes = true,
                BallEvents::NoBall => outcome.no_ball = true,
                BallEvents::Wicket => outcome.wicket = true,
                BallEvents::Wide => outcome.wide = true,
                BallEvents::Four => outcome.four = true,
                BallEvents::Six => outcome.six = true,
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
        if self.byes && self.leg_byes {
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

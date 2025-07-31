use super::player::Player;

use crate::error::BallOutcomeValidation;

#[derive(Debug, PartialEq, Clone)]
pub struct Wicket {
    pub player_out: String,
    pub kind: String,
}

#[derive(PartialEq)]
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

#[derive(Default, Debug)]
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
    pub penalty: Option<i32>,
}

impl BallOutcome {
    #[must_use]
    pub fn new(
        runs: i32,
        ball_events: Vec<BallEvents>,
        on_strike: Player,
        off_strike: Player,
    ) -> BallOutcome {
        let mut outcome = BallOutcome {
            runs,
            on_strike,
            off_strike,
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

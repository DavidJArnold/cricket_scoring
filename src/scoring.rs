pub mod innings;
pub mod player;

use std::fmt;

use player::Player;

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

#[derive(Default, Clone)]
pub struct CurrentScore {
    pub wickets_left: i32,
    pub wickets_lost: i32,
    pub runs: i32,
    pub leg_byes: i32,
    pub byes: i32,
    pub wides: i32,
    pub no_balls: i32,
    pub over: i32,
    pub ball: i32,
}

impl CurrentScore {
    #[must_use]
    pub fn new() -> CurrentScore {
        CurrentScore {
            ..CurrentScore::default()
        }
    }

    pub fn score_ball(&mut self, ball_outcome: &BallOutcome) {
        if !ball_outcome.wide && !ball_outcome.no_ball {
            self.ball += 1;
        }
        self.runs += ball_outcome.runs;
        if ball_outcome.wicket {
            self.wickets_lost += 1;
        }
        if ball_outcome.wide {
            self.wides += 1 + ball_outcome.runs;
            self.runs += 1;
        }
        if ball_outcome.no_ball {
            self.no_balls += 1;
            self.runs += 1;
        }
        if ball_outcome.byes {
            self.byes += ball_outcome.runs;
        }
        if ball_outcome.leg_byes {
            self.leg_byes += ball_outcome.runs;
        }
    }

    pub fn over(&mut self) {
        self.over += 1;
        self.ball = 0;
    }

    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "{}/{}\n{} wides, {} no balls, {} byes, {} leg byes\n{}.{}",
            self.wickets_lost,
            self.runs,
            self.wides,
            self.no_balls,
            self.byes,
            self.leg_byes,
            self.over,
            self.ball,
        )
    }
}

impl fmt::Display for CurrentScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{} ({}.{} overs)",
            self.wickets_lost, self.runs, self.over, self.ball
        )
    }
}

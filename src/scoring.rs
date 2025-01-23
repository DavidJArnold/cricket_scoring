pub mod innings;
pub mod player;
pub mod game;
pub mod ball;

use std::fmt;

pub use ball::BallOutcome;

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

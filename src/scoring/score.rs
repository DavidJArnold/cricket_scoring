use std::fmt;

pub use super::ball::BallOutcome;

#[derive(Default, Clone, Debug)]
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
            wickets_left: 10,
            ..CurrentScore::default()
        }
    }

    pub fn score_ball(&mut self, ball_outcome: &BallOutcome) {
        if ball_outcome.wide.is_none() && ball_outcome.no_ball.is_none() {
            self.ball += 1;
        }
        self.runs += ball_outcome.runs;
        if ball_outcome.wicket.is_some() {
            for wicket in ball_outcome.wicket.clone().unwrap() {
                if wicket.kind == "retired out" || !wicket.kind.contains("retired") {
                    self.wickets_lost += 1;
                    self.wickets_left -= 1;
                }
            }
        }
        if ball_outcome.wide.is_some() {
            self.wides += ball_outcome.wide.unwrap() + ball_outcome.runs;
            self.runs += ball_outcome.wide.unwrap();
        }
        if ball_outcome.no_ball.is_some() {
            self.no_balls += ball_outcome.no_ball.unwrap();
            self.runs += ball_outcome.no_ball.unwrap();
        }
        if ball_outcome.byes.is_some() {
            self.byes += ball_outcome.byes.unwrap();
            self.runs += ball_outcome.byes.unwrap();
        }
        if ball_outcome.leg_byes.is_some() {
            self.leg_byes += ball_outcome.leg_byes.unwrap();
            self.runs += ball_outcome.leg_byes.unwrap();
        }
        if ball_outcome.penalty.is_some() {
            self.runs += ball_outcome.penalty.unwrap();
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

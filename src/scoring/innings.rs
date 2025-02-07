use std::fmt;

use super::{player::Team, BallOutcome, CurrentScore};

pub struct Innings {
    pub score: CurrentScore,
    pub batting_team: Team,
    pub bowling_team: Team,
    pub on_strike: usize,
    pub off_strike: usize,
    pub finished: bool,
}

impl Innings {
    #[must_use]
    pub fn new(batting_team: Team, bowling_team: Team) -> Innings {
        Innings {
            score: CurrentScore::new(),
            batting_team,
            bowling_team,
            on_strike: 0,
            off_strike: 1,
            finished: false,
        }
    }

    pub fn over(&mut self) {
        self.score.over();
        (self.on_strike, self.off_strike) = (self.off_strike, self.on_strike);
    }

    /// # Panics
    ///
    /// Will panic if the `on_strike` player isn't part of the team
    /// This shouldn't happen...
    pub fn score_ball(&mut self, ball_outcome: &BallOutcome) {
        self.score.score_ball(ball_outcome);
        let striker = self.batting_team.players.get_mut(self.on_strike).unwrap();
        if !ball_outcome.wide && !ball_outcome.no_ball {
            striker.balls_faced += 1;
        }
        striker.runs += ball_outcome.runs;
        if ball_outcome.four {
            striker.fours += 1;
        }
        if ball_outcome.six {
            striker.sixes += 1;
        }
        if ball_outcome.wicket {
            striker.out = true;
            self.on_strike = self.on_strike.max(self.off_strike) + 1;
            if self.on_strike >= self.batting_team.players.len() {
                self.finished = true;
            }
        }
        if ball_outcome.runs % 2 == 1 {
            (self.on_strike, self.off_strike) = (self.off_strike, self.on_strike);
        }
    }
}

impl fmt::Display for Innings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut batters = String::new();
        for batter in self.batting_team.players.clone() {
            if batter.out || batter.balls_faced != 0 {
                // ony show batters who batted
                batters.push_str(&format!("{batter}"));
                batters.push('\n');
            }
        }
        write!(f, "{}\n{}", self.score.summary(), batters)
    }
}

use std::fmt;

use super::{player::Team, score::BallOutcome, score::CurrentScore};

#[derive(Clone, Debug)]
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

        if ball_outcome.wide.is_none() && ball_outcome.no_ball.is_none() {
            striker.balls_faced += 1;
            if ball_outcome.byes.is_none() && ball_outcome.leg_byes.is_none() {
                striker.runs += ball_outcome.runs;
                if ball_outcome.four {
                    striker.fours += 1;
                }
                if ball_outcome.six {
                    striker.sixes += 1;
                }
            }
        }

        if ball_outcome.runs % 2 == 1 {
            (self.on_strike, self.off_strike) = (self.off_strike, self.on_strike);
        }

        if ball_outcome.wicket.is_some() {
            for wicket in ball_outcome.wicket.as_ref().unwrap() {
                let player_out = wicket.player_out.clone();
                let out_striker = self.batting_team.players.get_mut(self.on_strike).unwrap();
                if player_out.contains(&out_striker.name) {
                    out_striker.out = true;
                    self.on_strike = self.on_strike.max(self.off_strike) + 1;
                } else {
                    let non_striker = self.batting_team.players.get_mut(self.off_strike).unwrap();
                    non_striker.out = true;
                    self.off_strike = self.on_strike.max(self.off_strike) + 1;
                };
            }
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

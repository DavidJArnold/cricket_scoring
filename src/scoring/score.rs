use serde::{Deserialize, Serialize};
use std::fmt;

pub use super::ball::BallOutcome;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scoring::ball::Wicket;

    fn create_test_wicket(kind: &str) -> Wicket {
        Wicket {
            player_out: "Test Player".to_string(),
            kind: kind.to_string(),
        }
    }

    #[test]
    fn test_current_score_new() {
        let score = CurrentScore::new();
        assert_eq!(score.wickets_left, 10);
        assert_eq!(score.wickets_lost, 0);
        assert_eq!(score.runs, 0);
        assert_eq!(score.leg_byes, 0);
        assert_eq!(score.byes, 0);
        assert_eq!(score.wides, 0);
        assert_eq!(score.no_balls, 0);
        assert_eq!(score.over, 0);
        assert_eq!(score.ball, 0);
    }

    #[test]
    fn test_current_score_default() {
        let score = CurrentScore::default();
        assert_eq!(score.wickets_left, 0); // Default is 0, not 10
        assert_eq!(score.wickets_lost, 0);
        assert_eq!(score.runs, 0);
        assert_eq!(score.leg_byes, 0);
        assert_eq!(score.byes, 0);
        assert_eq!(score.wides, 0);
        assert_eq!(score.no_balls, 0);
        assert_eq!(score.over, 0);
        assert_eq!(score.ball, 0);
    }

    #[test]
    fn test_current_score_clone() {
        let mut score = CurrentScore::new();
        score.runs = 50;
        score.wickets_lost = 3;
        score.over = 10;
        score.ball = 4;

        let cloned = score.clone();
        assert_eq!(score.runs, cloned.runs);
        assert_eq!(score.wickets_lost, cloned.wickets_lost);
        assert_eq!(score.wickets_left, cloned.wickets_left);
        assert_eq!(score.over, cloned.over);
        assert_eq!(score.ball, cloned.ball);
    }

    fn create_test_ball_outcome() -> BallOutcome {
        use crate::scoring::player::Player;
        BallOutcome {
            on_strike: Player::new("Striker".to_string()),
            off_strike: Player::new("NonStriker".to_string()),
            bowler: Player::new("Bowler".to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn test_score_ball_simple_runs() {
        let mut score = CurrentScore::new();
        let ball_outcome = BallOutcome {
            runs: 2,
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.runs, 2);
        assert_eq!(score.ball, 1);
        assert_eq!(score.wickets_lost, 0);
    }

    #[test]
    fn test_score_ball_with_wicket() {
        let mut score = CurrentScore::new();
        let wicket = vec![create_test_wicket("bowled")];
        let ball_outcome = BallOutcome {
            runs: 0,
            wicket: Some(wicket),
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.runs, 0);
        assert_eq!(score.ball, 1);
        assert_eq!(score.wickets_lost, 1);
        assert_eq!(score.wickets_left, 9);
    }

    #[test]
    fn test_score_ball_with_wide() {
        let mut score = CurrentScore::new();
        let ball_outcome = BallOutcome {
            runs: 2,
            wide: Some(1),
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.runs, 3); // 2 runs + 1 wide
        assert_eq!(score.wides, 3); // 1 wide + 2 runs
        assert_eq!(score.ball, 0); // Wide doesn't advance ball count
    }

    #[test]
    fn test_score_ball_with_no_ball() {
        let mut score = CurrentScore::new();
        let ball_outcome = BallOutcome {
            runs: 1,
            no_ball: Some(1),
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.runs, 2); // 1 run + 1 no ball
        assert_eq!(score.no_balls, 1);
        assert_eq!(score.ball, 0); // No ball doesn't advance ball count
    }

    #[test]
    fn test_score_ball_with_byes() {
        let mut score = CurrentScore::new();
        let ball_outcome = BallOutcome {
            runs: 2,
            byes: Some(2),
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.runs, 4); // 2 runs + 2 byes
        assert_eq!(score.byes, 2);
        assert_eq!(score.ball, 1);
    }

    #[test]
    fn test_score_ball_with_leg_byes() {
        let mut score = CurrentScore::new();
        let ball_outcome = BallOutcome {
            runs: 1,
            leg_byes: Some(1),
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.runs, 2); // 1 run + 1 leg bye
        assert_eq!(score.leg_byes, 1);
        assert_eq!(score.ball, 1);
    }

    #[test]
    fn test_score_ball_with_penalty() {
        let mut score = CurrentScore::new();
        let ball_outcome = BallOutcome {
            runs: 0,
            penalty: Some(5),
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.runs, 5);
        assert_eq!(score.ball, 1);
    }

    #[test]
    fn test_score_ball_complex() {
        let mut score = CurrentScore::new();
        let wicket = vec![create_test_wicket("caught")];
        let ball_outcome = BallOutcome {
            runs: 1,
            wicket: Some(wicket),
            no_ball: Some(1),
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.runs, 2); // 1 run + 1 no ball
        assert_eq!(score.no_balls, 1);
        assert_eq!(score.wickets_lost, 1);
        assert_eq!(score.wickets_left, 9);
        assert_eq!(score.ball, 0); // No ball doesn't advance ball count
    }

    #[test]
    fn test_score_ball_retired_wicket() {
        let mut score = CurrentScore::new();
        let wicket = vec![create_test_wicket("retired hurt")];
        let ball_outcome = BallOutcome {
            runs: 0,
            wicket: Some(wicket),
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.wickets_lost, 0); // Retired hurt shouldn't count as wicket lost
        assert_eq!(score.wickets_left, 10);
    }

    #[test]
    fn test_score_ball_retired_out_wicket() {
        let mut score = CurrentScore::new();
        let wicket = vec![create_test_wicket("retired out")];
        let ball_outcome = BallOutcome {
            runs: 0,
            wicket: Some(wicket),
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.wickets_lost, 1); // Retired out should count as wicket lost
        assert_eq!(score.wickets_left, 9);
    }

    #[test]
    fn test_over() {
        let mut score = CurrentScore::new();

        // Simulate 6 balls
        for _ in 0..6 {
            let ball_outcome = BallOutcome {
                runs: 1,
                ..create_test_ball_outcome()
            };
            score.score_ball(&ball_outcome);
        }

        assert_eq!(score.ball, 6);
        assert_eq!(score.over, 0);

        // Call over
        score.over();

        assert_eq!(score.ball, 0);
        assert_eq!(score.over, 1);
    }

    #[test]
    fn test_summary() {
        let mut score = CurrentScore::new();
        score.wickets_lost = 3;
        score.runs = 125;
        score.wides = 5;
        score.no_balls = 2;
        score.byes = 3;
        score.leg_byes = 1;
        score.over = 20;
        score.ball = 3;

        let summary = score.summary();
        assert_eq!(
            summary,
            "3/125\n5 wides, 2 no balls, 3 byes, 1 leg byes\n20.3"
        );
    }

    #[test]
    fn test_display() {
        let mut score = CurrentScore::new();
        score.wickets_lost = 2;
        score.runs = 45;
        score.over = 10;
        score.ball = 2;

        let display = format!("{}", score);
        assert_eq!(display, "2/45 (10.2 overs)");
    }

    #[test]
    fn test_multiple_wickets_same_ball() {
        let mut score = CurrentScore::new();
        let wickets = vec![create_test_wicket("run out"), create_test_wicket("bowled")];
        let ball_outcome = BallOutcome {
            runs: 0,
            wicket: Some(wickets),
            ..create_test_ball_outcome()
        };

        score.score_ball(&ball_outcome);

        assert_eq!(score.wickets_lost, 2);
        assert_eq!(score.wickets_left, 8);
    }

    #[test]
    fn test_progression_through_over() {
        let mut score = CurrentScore::new();

        // Score through multiple balls and overs
        for over in 0..5 {
            for ball in 0..6 {
                let ball_outcome = BallOutcome {
                    runs: 1,
                    ..create_test_ball_outcome()
                };
                score.score_ball(&ball_outcome);

                assert_eq!(score.over, over);
                assert_eq!(score.ball, ball + 1);
            }
            score.over();
        }

        assert_eq!(score.over, 5);
        assert_eq!(score.ball, 0);
        assert_eq!(score.runs, 30); // 5 overs * 6 balls * 1 run
    }

    #[test]
    fn test_zero_values_summary() {
        let score = CurrentScore::new();
        let summary = score.summary();
        assert_eq!(summary, "0/0\n0 wides, 0 no balls, 0 byes, 0 leg byes\n0.0");
    }
}

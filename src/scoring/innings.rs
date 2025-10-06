use serde::{Deserialize, Serialize};
use std::fmt;

use super::{player::Team, score::BallOutcome, score::CurrentScore};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    /// Will panic if the `on_strike` player isn't part of the team or if the bowler isn't found in the bowling team
    /// This shouldn't happen...
    pub fn score_ball(&mut self, ball_outcome: &BallOutcome) {
        self.score.score_ball(ball_outcome);

        // Find the striker by name from the BallOutcome (source of truth)
        let striker_index = self
            .batting_team
            .players
            .iter()
            .position(|p| p.name == ball_outcome.on_strike.name)
            .expect("Striker from BallOutcome not found in batting team");

        // Find the non-striker by name from the BallOutcome (source of truth)
        let non_striker_index = self
            .batting_team
            .players
            .iter()
            .position(|p| p.name == ball_outcome.off_strike.name)
            .expect("Non-striker from BallOutcome not found in batting team");

        // Update tracked indices to match reality
        self.on_strike = striker_index;
        self.off_strike = non_striker_index;

        // Update batting stats
        let striker = self.batting_team.players.get_mut(striker_index).unwrap();

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

        // Update bowling stats
        if let Some(bowler) = self
            .bowling_team
            .players
            .iter_mut()
            .find(|p| p.name == ball_outcome.bowler.name)
        {
            // Count legal deliveries (not wides or no balls)
            if ball_outcome.wide.is_none() && ball_outcome.no_ball.is_none() {
                bowler.balls_bowled += 1;
            }

            // Track runs conceded (including byes and leg byes count as runs conceded)
            bowler.runs_conceded += ball_outcome.runs;

            // Track wickets
            if let Some(wickets) = &ball_outcome.wicket {
                bowler.wickets_taken += wickets.len() as i32;
            }

            // Track wides and no balls
            if ball_outcome.wide.is_some() {
                bowler.wides += 1;
            }
            if ball_outcome.no_ball.is_some() {
                bowler.no_balls += 1;
            }
        }

        if ball_outcome.runs % 2 == 1 {
            (self.on_strike, self.off_strike) = (self.off_strike, self.on_strike);
        }

        if ball_outcome.wicket.is_some() {
            for wicket in ball_outcome.wicket.as_ref().unwrap() {
                // Find the player who got out by matching the name
                let out_player_index = self
                    .batting_team
                    .players
                    .iter()
                    .position(|p| wicket.player_out.contains(&p.name))
                    .expect("Player from wicket not found in batting team");

                let out_player = self.batting_team.players.get_mut(out_player_index).unwrap();
                out_player.out = true;
                out_player.dismissal = Some(wicket.kind.clone());

                // Bring in next batsman based on who got out
                if out_player_index == self.on_strike {
                    self.on_strike = self.on_strike.max(self.off_strike) + 1;
                } else {
                    self.off_strike = self.on_strike.max(self.off_strike) + 1;
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scoring::ball::{BallEvents, Wicket};
    use crate::scoring::player::Player;

    fn create_test_team(name: &str) -> Team {
        Team {
            name: name.to_string(),
            players: vec![
                Player::new("Player1".to_string()),
                Player::new("Player2".to_string()),
                Player::new("Player3".to_string()),
                Player::new("Player4".to_string()),
                Player::new("Player5".to_string()),
            ],
        }
    }

    fn create_test_ball_outcome(
        runs: i32,
        events: Vec<BallEvents>,
        striker: Player,
        non_striker: Player,
    ) -> BallOutcome {
        BallOutcome::new(
            runs,
            events,
            striker,
            non_striker,
            Player::new("Bowler".to_string()),
        )
    }

    #[test]
    fn test_innings_new() {
        let batting_team = create_test_team("Batting Team");
        let bowling_team = create_test_team("Bowling Team");

        let innings = Innings::new(batting_team.clone(), bowling_team.clone());

        assert_eq!(innings.batting_team.name, "Batting Team");
        assert_eq!(innings.bowling_team.name, "Bowling Team");
        assert_eq!(innings.on_strike, 0);
        assert_eq!(innings.off_strike, 1);
        assert!(!innings.finished);
        assert_eq!(innings.score.runs, 0);
        assert_eq!(innings.score.wickets_left, 10);
    }

    #[test]
    fn test_innings_clone() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team, bowling_team);

        innings.score.runs = 50;
        innings.on_strike = 2;
        innings.finished = true;

        let cloned = innings.clone();
        assert_eq!(innings.batting_team.name, cloned.batting_team.name);
        assert_eq!(innings.bowling_team.name, cloned.bowling_team.name);
        assert_eq!(innings.score.runs, cloned.score.runs);
        assert_eq!(innings.on_strike, cloned.on_strike);
        assert_eq!(innings.finished, cloned.finished);
    }

    #[test]
    fn test_over() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team, bowling_team);

        assert_eq!(innings.on_strike, 0);
        assert_eq!(innings.off_strike, 1);

        innings.over();

        assert_eq!(innings.on_strike, 1);
        assert_eq!(innings.off_strike, 0);
        assert_eq!(innings.score.over, 1);
        assert_eq!(innings.score.ball, 0);
    }

    #[test]
    fn test_score_ball_simple() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        let ball_outcome = create_test_ball_outcome(
            1,
            vec![],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        assert_eq!(innings.score.runs, 1);
        assert_eq!(innings.score.ball, 1);
        assert_eq!(innings.batting_team.players[0].balls_faced, 1);
        assert_eq!(innings.batting_team.players[0].runs, 1);
    }

    #[test]
    fn test_score_ball_four() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        let ball_outcome = create_test_ball_outcome(
            4,
            vec![BallEvents::Four],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        assert_eq!(innings.score.runs, 4);
        assert_eq!(innings.batting_team.players[0].runs, 4);
        assert_eq!(innings.batting_team.players[0].fours, 1);
        assert_eq!(innings.batting_team.players[0].sixes, 0);
    }

    #[test]
    fn test_score_ball_six() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        let ball_outcome = create_test_ball_outcome(
            6,
            vec![BallEvents::Six],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        assert_eq!(innings.score.runs, 6);
        assert_eq!(innings.batting_team.players[0].runs, 6);
        assert_eq!(innings.batting_team.players[0].fours, 0);
        assert_eq!(innings.batting_team.players[0].sixes, 1);
    }

    #[test]
    fn test_score_ball_wide() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        let ball_outcome = create_test_ball_outcome(
            1,
            vec![BallEvents::Wide(1)],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        assert_eq!(innings.score.runs, 2); // 1 run + 1 wide
        assert_eq!(innings.batting_team.players[0].balls_faced, 0); // Wide doesn't count as ball faced
        assert_eq!(innings.batting_team.players[0].runs, 0); // Wide runs don't count for batsman
    }

    #[test]
    fn test_score_ball_no_ball() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        let ball_outcome = create_test_ball_outcome(
            1,
            vec![BallEvents::NoBall(1)],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        assert_eq!(innings.score.runs, 2); // 1 run + 1 no ball
        assert_eq!(innings.batting_team.players[0].balls_faced, 0); // No ball doesn't count as ball faced
        assert_eq!(innings.batting_team.players[0].runs, 0); // No ball with runs but batsman doesn't face ball, so no runs credited
    }

    #[test]
    fn test_score_ball_byes() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        let ball_outcome = create_test_ball_outcome(
            2,
            vec![BallEvents::Bye(2)],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        assert_eq!(innings.score.runs, 4); // 2 runs + 2 byes
        assert_eq!(innings.batting_team.players[0].balls_faced, 1);
        assert_eq!(innings.batting_team.players[0].runs, 0); // Batsman doesn't get bye runs
    }

    #[test]
    fn test_score_ball_leg_byes() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        let ball_outcome = create_test_ball_outcome(
            1,
            vec![BallEvents::LegBye(1)],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        assert_eq!(innings.score.runs, 2); // 1 run + 1 leg bye
        assert_eq!(innings.batting_team.players[0].balls_faced, 1);
        assert_eq!(innings.batting_team.players[0].runs, 0); // Batsman doesn't get leg bye runs
    }

    #[test]
    fn test_score_ball_odd_runs_switch_strike() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        assert_eq!(innings.on_strike, 0);
        assert_eq!(innings.off_strike, 1);

        let ball_outcome = create_test_ball_outcome(
            1,
            vec![],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        // After odd runs, batsmen should switch
        assert_eq!(innings.on_strike, 1);
        assert_eq!(innings.off_strike, 0);
    }

    #[test]
    fn test_score_ball_even_runs_no_switch() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        assert_eq!(innings.on_strike, 0);
        assert_eq!(innings.off_strike, 1);

        let ball_outcome = create_test_ball_outcome(
            2,
            vec![],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        // After even runs, batsmen should not switch
        assert_eq!(innings.on_strike, 0);
        assert_eq!(innings.off_strike, 1);
    }

    #[test]
    fn test_score_ball_wicket_on_strike() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        let wicket = vec![Wicket {
            player_out: "Player1".to_string(),
            kind: "bowled".to_string(),
        }];
        let ball_outcome = create_test_ball_outcome(
            0,
            vec![BallEvents::Wicket(wicket)],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        assert!(innings.batting_team.players[0].out);
        assert_eq!(
            innings.batting_team.players[0].dismissal,
            Some("bowled".to_string())
        );
        assert_eq!(innings.on_strike, 2); // Next batsman comes in
        assert_eq!(innings.off_strike, 1); // Non-striker stays
        assert_eq!(innings.score.wickets_lost, 1);
    }

    #[test]
    fn test_score_ball_wicket_off_strike() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        let wicket = vec![Wicket {
            player_out: "Player2".to_string(),
            kind: "run out".to_string(),
        }];
        let ball_outcome = create_test_ball_outcome(
            0,
            vec![BallEvents::Wicket(wicket)],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        assert!(!innings.batting_team.players[0].out); // On-strike batsman is fine
        assert_eq!(innings.batting_team.players[0].dismissal, None); // No dismissal for on-strike
        assert!(innings.batting_team.players[1].out); // Off-strike batsman is out
        assert_eq!(
            innings.batting_team.players[1].dismissal,
            Some("run out".to_string())
        );
        assert_eq!(innings.on_strike, 0); // Striker stays
        assert_eq!(innings.off_strike, 2); // Next batsman comes in
        assert_eq!(innings.score.wickets_lost, 1);
    }

    #[test]
    fn test_display_empty_innings() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let innings = Innings::new(batting_team, bowling_team);

        let display = format!("{}", innings);
        assert!(display.contains("0/0"));
        assert!(display.contains("0 wides, 0 no balls, 0 byes, 0 leg byes"));
        assert!(display.contains("0.0"));
    }

    #[test]
    fn test_display_with_batting() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        // Score some runs
        let ball_outcome = create_test_ball_outcome(
            4,
            vec![BallEvents::Four],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball_outcome);

        let display = format!("{}", innings);
        assert!(display.contains("0/4")); // Score
        assert!(display.contains("Player1: Batting: 4*(1), 1 4s, 0 6s, SR: 400.00"));
        // Player who batted
    }

    #[test]
    fn test_complex_scoring_scenario() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        // Ball 1: 4 runs with boundary (Player1 on strike)
        let ball1 = create_test_ball_outcome(
            4,
            vec![BallEvents::Four],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball1);

        // Ball 2: 6 runs with six (Player1 on strike)
        let ball2 = create_test_ball_outcome(
            6,
            vec![BallEvents::Six],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball2);

        // Ball 3: 1 run (Player1 on strike, switches to Player2)
        let ball3 = create_test_ball_outcome(
            1,
            vec![],
            batting_team.players[0].clone(),
            batting_team.players[1].clone(),
        );
        innings.score_ball(&ball3);

        // Ball 4: Wide (Player2 on strike, no strike change)
        let ball4 = create_test_ball_outcome(
            1,
            vec![BallEvents::Wide(1)],
            batting_team.players[1].clone(),
            batting_team.players[0].clone(),
        );
        innings.score_ball(&ball4);

        // Ball 5: Wicket (Player2 gets out)
        let wicket = vec![Wicket {
            player_out: "Player2".to_string(),
            kind: "caught".to_string(),
        }];
        let ball5 = create_test_ball_outcome(
            0,
            vec![BallEvents::Wicket(wicket)],
            batting_team.players[1].clone(),
            batting_team.players[0].clone(),
        );
        innings.score_ball(&ball5);

        // Verify final state
        assert_eq!(innings.score.runs, 13); // 4+6+1+1+1+0
        assert_eq!(innings.score.ball, 4); // 4 valid balls (wide doesn't count)
        assert_eq!(innings.score.wickets_lost, 1);
        assert_eq!(innings.batting_team.players[0].runs, 11); // 4+6+1 from first batsman
        assert_eq!(innings.batting_team.players[0].fours, 1);
        assert_eq!(innings.batting_team.players[0].sixes, 1);
        assert_eq!(innings.batting_team.players[1].runs, 0); // Second batsman didn't score any runs
        assert!(innings.batting_team.players[1].out); // Second batsman is out
        assert_eq!(innings.on_strike, 2); // Third batsman comes in on strike
        assert_eq!(innings.off_strike, 0); // First batsman at non-striker
    }

    #[test]
    fn test_multiple_overs() {
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        // Complete first over - alternating singles means strike switches each ball
        for i in 0..6 {
            let on_strike_player = if i % 2 == 0 { 0 } else { 1 };
            let off_strike_player = if i % 2 == 0 { 1 } else { 0 };
            let ball_outcome = create_test_ball_outcome(
                1,
                vec![],
                batting_team.players[on_strike_player].clone(),
                batting_team.players[off_strike_player].clone(),
            );
            innings.score_ball(&ball_outcome);
        }
        innings.over();

        // Complete second over - after over(), strike switches, so P1 is on strike again
        for i in 0..6 {
            let on_strike_player = if i % 2 == 0 { 0 } else { 1 };
            let off_strike_player = if i % 2 == 0 { 1 } else { 0 };
            let ball_outcome = create_test_ball_outcome(
                1,
                vec![],
                batting_team.players[on_strike_player].clone(),
                batting_team.players[off_strike_player].clone(),
            );
            innings.score_ball(&ball_outcome);
        }
        innings.over();

        assert_eq!(innings.score.runs, 12);
        assert_eq!(innings.score.over, 2);
        assert_eq!(innings.score.ball, 0);
    }

    #[test]
    fn test_score_ball_trusts_ball_outcome_not_indices() {
        // This test verifies the fix for the batter scoring bug
        // Even if tracked indices are "wrong", the generic layer should trust BallOutcome player names
        let batting_team = create_test_team("Team A");
        let bowling_team = create_test_team("Team B");
        let mut innings = Innings::new(batting_team.clone(), bowling_team);

        // Manually set wrong indices (simulating drift/desync)
        innings.on_strike = 2;
        innings.off_strike = 3;

        // But BallOutcome says Player1 and Player2 are actually batting
        let ball_outcome = create_test_ball_outcome(
            4,
            vec![BallEvents::Four],
            batting_team.players[0].clone(), // Player1 is on strike according to BallOutcome
            batting_team.players[1].clone(), // Player2 is off strike according to BallOutcome
        );
        innings.score_ball(&ball_outcome);

        // Verify runs are credited to Player1 (from BallOutcome), not Player3 (from indices)
        assert_eq!(innings.batting_team.players[0].runs, 4); // Player1 gets the runs
        assert_eq!(innings.batting_team.players[0].fours, 1);
        assert_eq!(innings.batting_team.players[0].balls_faced, 1);
        assert_eq!(innings.batting_team.players[2].runs, 0); // Player3 gets nothing
        assert_eq!(innings.batting_team.players[2].balls_faced, 0);

        // Verify indices are now corrected to match reality
        assert_eq!(innings.on_strike, 0); // Corrected to Player1's index
        assert_eq!(innings.off_strike, 1); // Corrected to Player2's index
    }
}

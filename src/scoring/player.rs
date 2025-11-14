use core::fmt;
use serde::{Deserialize, Serialize};

/// Represents a cricket player with their batting and bowling statistics.
///
/// # Fields
///
/// ## Batting Stats
/// * `name` - The player's name
/// * `runs` - Total runs scored
/// * `balls_faced` - Number of balls faced
/// * `fours` - Number of boundaries (4 runs)
/// * `sixes` - Number of sixes (6 runs)
/// * `out` - Whether the player is out
/// * `dismissal` - The type of dismissal if the player is out (e.g., "bowled", "caught", "run out")
///
/// ## Bowling Stats
/// * `balls_bowled` - Number of legal deliveries bowled
/// * `runs_conceded` - Total runs conceded while bowling
/// * `wickets_taken` - Number of wickets taken
/// * `maidens` - Number of maiden overs bowled
/// * `wides` - Number of wides bowled
/// * `no_balls` - Number of no balls bowled
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    // Batting stats
    pub runs: i32,
    pub balls_faced: i32,
    pub fours: i32,
    pub sixes: i32,
    pub out: bool,
    /// The method of dismissal (e.g., "bowled", "caught", "lbw", "run out").
    /// Set to `None` if the player is not out.
    pub dismissal: Option<String>,
    // Bowling stats
    pub balls_bowled: i32,
    pub runs_conceded: i32,
    pub wickets_taken: i32,
    pub maidens: i32,
    pub wides: i32,
    pub no_balls: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub players: Vec<Player>,
    pub name: String,
}

impl Player {
    #[must_use]
    pub fn new(name: String) -> Self {
        Player {
            name,
            ..Default::default()
        }
    }

    /// Calculate the player's strike rate (runs per 100 balls)
    /// Returns None if the player has not faced any balls
    #[must_use]
    pub fn strike_rate(&self) -> Option<f64> {
        if self.balls_faced == 0 {
            None
        } else {
            Some((self.runs as f64 / self.balls_faced as f64) * 100.0)
        }
    }

    /// Calculate the player's bowling economy rate (runs per over)
    /// Returns None if the player has not bowled any balls
    #[must_use]
    pub fn economy_rate(&self) -> Option<f64> {
        if self.balls_bowled == 0 {
            None
        } else {
            let overs = self.balls_bowled as f64 / 6.0;
            Some(self.runs_conceded as f64 / overs)
        }
    }

    /// Calculate the player's bowling average (runs per wicket)
    /// Returns None if the player has not taken any wickets
    #[must_use]
    pub fn bowling_average(&self) -> Option<f64> {
        if self.wickets_taken == 0 {
            None
        } else {
            Some(self.runs_conceded as f64 / self.wickets_taken as f64)
        }
    }

    /// Calculate the player's bowling strike rate (balls per wicket)
    /// Returns None if the player has not taken any wickets
    #[must_use]
    pub fn bowling_strike_rate(&self) -> Option<f64> {
        if self.wickets_taken == 0 {
            None
        } else {
            Some(self.balls_bowled as f64 / self.wickets_taken as f64)
        }
    }

    /// Get the number of complete overs bowled
    #[must_use]
    pub fn overs_bowled(&self) -> (i32, i32) {
        let complete_overs = self.balls_bowled / 6;
        let remaining_balls = self.balls_bowled % 6;
        (complete_overs, remaining_balls)
    }
}

impl Team {
    pub fn get_player_index(&self, player_name: &str) -> Option<usize> {
        self.players.iter().position(|p| p.name == player_name)
    }

    pub fn get_player(&mut self, player_name: &str) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.name == player_name)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts = Vec::new();

        // Batting stats (if player has batted)
        if self.balls_faced > 0 || self.out {
            let mut not_out_ind = "";
            if !self.out {
                not_out_ind = "*";
            }

            let strike_rate_str = match self.strike_rate() {
                Some(sr) => format!(", SR: {:.2}", sr),
                None => String::new(),
            };

            parts.push(format!(
                "Batting: {}{}({}), {} 4s, {} 6s{}",
                self.runs, not_out_ind, self.balls_faced, self.fours, self.sixes, strike_rate_str
            ));
        }

        // Bowling stats (if player has bowled)
        if self.balls_bowled > 0 {
            let (overs, balls) = self.overs_bowled();
            let overs_str = if balls == 0 {
                format!("{}", overs)
            } else {
                format!("{}.{}", overs, balls)
            };

            let economy_str = match self.economy_rate() {
                Some(econ) => format!(", Econ: {:.2}", econ),
                None => String::new(),
            };

            parts.push(format!(
                "Bowling: {}-{} ({} overs), {} maidens, {} wides, {} no balls{}",
                self.wickets_taken,
                self.runs_conceded,
                overs_str,
                self.maidens,
                self.wides,
                self.no_balls,
                economy_str
            ));
        }

        // If player has neither batted nor bowled, show just the name
        if parts.is_empty() {
            write!(f, "{}: No stats", self.name)
        } else {
            write!(f, "{}: {}", self.name, parts.join("; "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_new() {
        let player = Player::new("John Doe".to_string());
        assert_eq!(player.name, "John Doe");
        assert_eq!(player.runs, 0);
        assert_eq!(player.balls_faced, 0);
        assert_eq!(player.fours, 0);
        assert_eq!(player.sixes, 0);
        assert!(!player.out);
        assert_eq!(player.dismissal, None);
    }

    #[test]
    fn test_player_default() {
        let player = Player::default();
        assert_eq!(player.name, "");
        assert_eq!(player.runs, 0);
        assert_eq!(player.balls_faced, 0);
        assert_eq!(player.fours, 0);
        assert_eq!(player.sixes, 0);
        assert!(!player.out);
        assert_eq!(player.dismissal, None);
    }

    #[test]
    fn test_player_clone() {
        let mut player = Player::new("Jane Smith".to_string());
        player.runs = 50;
        player.balls_faced = 30;
        player.fours = 6;
        player.sixes = 2;
        player.out = true;
        player.dismissal = Some("caught".to_string());

        let cloned = player.clone();
        assert_eq!(player.name, cloned.name);
        assert_eq!(player.runs, cloned.runs);
        assert_eq!(player.balls_faced, cloned.balls_faced);
        assert_eq!(player.fours, cloned.fours);
        assert_eq!(player.sixes, cloned.sixes);
        assert_eq!(player.out, cloned.out);
        assert_eq!(player.dismissal, cloned.dismissal);
    }

    #[test]
    fn test_player_display_not_out() {
        let mut player = Player::new("Bob Wilson".to_string());
        player.runs = 45;
        player.balls_faced = 30;
        player.fours = 5;
        player.sixes = 1;
        player.out = false;

        let display = format!("{}", player);
        assert_eq!(
            display,
            "Bob Wilson: Batting: 45*(30), 5 4s, 1 6s, SR: 150.00"
        );
    }

    #[test]
    fn test_player_display_out() {
        let mut player = Player::new("Alice Brown".to_string());
        player.runs = 25;
        player.balls_faced = 20;
        player.fours = 3;
        player.sixes = 0;
        player.out = true;

        let display = format!("{}", player);
        assert_eq!(
            display,
            "Alice Brown: Batting: 25(20), 3 4s, 0 6s, SR: 125.00"
        );
    }

    #[test]
    fn test_player_display_zero_stats() {
        let player = Player::new("New Player".to_string());
        let display = format!("{}", player);
        assert_eq!(display, "New Player: No stats");
    }

    #[test]
    fn test_team_creation() {
        let players = vec![
            Player::new("Player1".to_string()),
            Player::new("Player2".to_string()),
            Player::new("Player3".to_string()),
        ];

        let team = Team {
            name: "Test Team".to_string(),
            players: players.clone(),
        };

        assert_eq!(team.name, "Test Team");
        assert_eq!(team.players.len(), 3);
        assert_eq!(team.players[0].name, "Player1");
        assert_eq!(team.players[1].name, "Player2");
        assert_eq!(team.players[2].name, "Player3");
    }

    #[test]
    fn test_team_clone() {
        let players = vec![
            Player::new("Player1".to_string()),
            Player::new("Player2".to_string()),
        ];

        let team = Team {
            name: "Original Team".to_string(),
            players,
        };

        let cloned_team = team.clone();
        assert_eq!(team.name, cloned_team.name);
        assert_eq!(team.players.len(), cloned_team.players.len());
        assert_eq!(team.players[0].name, cloned_team.players[0].name);
        assert_eq!(team.players[1].name, cloned_team.players[1].name);
    }

    #[test]
    fn test_team_empty_players() {
        let team = Team {
            name: "Empty Team".to_string(),
            players: vec![],
        };

        assert_eq!(team.name, "Empty Team");
        assert_eq!(team.players.len(), 0);
    }

    #[test]
    fn test_player_stats_modification() {
        let mut player = Player::new("Test Batsman".to_string());

        // Simulate scoring
        player.runs += 4;
        player.balls_faced += 1;
        player.fours += 1;

        assert_eq!(player.runs, 4);
        assert_eq!(player.balls_faced, 1);
        assert_eq!(player.fours, 1);
        assert_eq!(player.sixes, 0);
        assert!(!player.out);

        // Simulate getting out
        player.out = true;
        assert!(player.out);
    }

    #[test]
    fn test_player_complex_scoring() {
        let mut player = Player::new("Heavy Scorer".to_string());

        // Simulate a complex innings
        player.runs = 125;
        player.balls_faced = 85;
        player.fours = 12;
        player.sixes = 4;
        player.out = true;

        let display = format!("{}", player);
        assert_eq!(
            display,
            "Heavy Scorer: Batting: 125(85), 12 4s, 4 6s, SR: 147.06"
        );
    }

    #[test]
    fn test_strike_rate_calculation() {
        let mut player = Player::new("Test Player".to_string());

        // Test with no balls faced
        assert_eq!(player.strike_rate(), None);

        // Test with 50 runs from 50 balls (strike rate 100)
        player.runs = 50;
        player.balls_faced = 50;
        assert_eq!(player.strike_rate(), Some(100.0));

        // Test with 75 runs from 50 balls (strike rate 150)
        player.runs = 75;
        player.balls_faced = 50;
        assert_eq!(player.strike_rate(), Some(150.0));

        // Test with 25 runs from 100 balls (strike rate 25)
        player.runs = 25;
        player.balls_faced = 100;
        assert_eq!(player.strike_rate(), Some(25.0));

        // Test with 1 run from 3 balls (strike rate 33.333...)
        player.runs = 1;
        player.balls_faced = 3;
        let sr = player.strike_rate().unwrap();
        assert!((sr - 33.333333333333336).abs() < 0.000001); // Check within floating point precision
    }

    #[test]
    fn test_strike_rate_display_formatting() {
        let mut player = Player::new("Test Batsman".to_string());

        // Test with exact strike rate
        player.runs = 50;
        player.balls_faced = 50;
        player.out = false;
        let display = format!("{}", player);
        assert_eq!(
            display,
            "Test Batsman: Batting: 50*(50), 0 4s, 0 6s, SR: 100.00"
        );

        // Test with decimal strike rate
        player.runs = 33;
        player.balls_faced = 25;
        let display = format!("{}", player);
        assert_eq!(
            display,
            "Test Batsman: Batting: 33*(25), 0 4s, 0 6s, SR: 132.00"
        );

        // Test with repeating decimal
        player.runs = 10;
        player.balls_faced = 3;
        let display = format!("{}", player);
        assert_eq!(
            display,
            "Test Batsman: Batting: 10*(3), 0 4s, 0 6s, SR: 333.33"
        );
    }

    #[test]
    fn test_strike_rate_edge_cases() {
        let mut player = Player::new("Edge Case".to_string());

        // Test with 0 runs but balls faced
        player.runs = 0;
        player.balls_faced = 10;
        assert_eq!(player.strike_rate(), Some(0.0));

        // Test with very high strike rate
        player.runs = 200;
        player.balls_faced = 100;
        assert_eq!(player.strike_rate(), Some(200.0));

        // Test with very low strike rate
        player.runs = 1;
        player.balls_faced = 100;
        assert_eq!(player.strike_rate(), Some(1.0));
    }

    // Bowling stats tests

    #[test]
    fn test_player_bowling_stats_default() {
        let player = Player::new("Bowler".to_string());
        assert_eq!(player.balls_bowled, 0);
        assert_eq!(player.runs_conceded, 0);
        assert_eq!(player.wickets_taken, 0);
        assert_eq!(player.maidens, 0);
        assert_eq!(player.wides, 0);
        assert_eq!(player.no_balls, 0);
    }

    #[test]
    fn test_economy_rate_calculation() {
        let mut player = Player::new("Fast Bowler".to_string());

        // Test with no balls bowled
        assert_eq!(player.economy_rate(), None);

        // Test with 1 over (6 balls), 4 runs conceded = 4 runs per over
        player.balls_bowled = 6;
        player.runs_conceded = 4;
        assert_eq!(player.economy_rate(), Some(4.0));

        // Test with 4 overs (24 balls), 20 runs conceded = 5 runs per over
        player.balls_bowled = 24;
        player.runs_conceded = 20;
        assert_eq!(player.economy_rate(), Some(5.0));

        // Test with partial over: 10 balls (1.4 overs), 15 runs = 9 runs per over
        player.balls_bowled = 10;
        player.runs_conceded = 15;
        let econ = player.economy_rate().unwrap();
        assert!((econ - 9.0).abs() < 0.000001);
    }

    #[test]
    fn test_bowling_average_calculation() {
        let mut player = Player::new("Spinner".to_string());

        // Test with no wickets
        assert_eq!(player.bowling_average(), None);

        // Test with 2 wickets for 30 runs = 15.0 average
        player.runs_conceded = 30;
        player.wickets_taken = 2;
        assert_eq!(player.bowling_average(), Some(15.0));

        // Test with 5 wickets for 25 runs = 5.0 average
        player.runs_conceded = 25;
        player.wickets_taken = 5;
        assert_eq!(player.bowling_average(), Some(5.0));

        // Test with 1 wicket for 0 runs = 0.0 average
        player.runs_conceded = 0;
        player.wickets_taken = 1;
        assert_eq!(player.bowling_average(), Some(0.0));
    }

    #[test]
    fn test_bowling_strike_rate_calculation() {
        let mut player = Player::new("Strike Bowler".to_string());

        // Test with no wickets
        assert_eq!(player.bowling_strike_rate(), None);

        // Test with 2 wickets in 12 balls = 6.0 balls per wicket
        player.balls_bowled = 12;
        player.wickets_taken = 2;
        assert_eq!(player.bowling_strike_rate(), Some(6.0));

        // Test with 5 wickets in 30 balls = 6.0 balls per wicket
        player.balls_bowled = 30;
        player.wickets_taken = 5;
        assert_eq!(player.bowling_strike_rate(), Some(6.0));

        // Test with 3 wickets in 10 balls = 3.333... balls per wicket
        player.balls_bowled = 10;
        player.wickets_taken = 3;
        let sr = player.bowling_strike_rate().unwrap();
        assert!((sr - 3.333333333333333).abs() < 0.000001);
    }

    #[test]
    fn test_overs_bowled() {
        let mut player = Player::new("Bowler".to_string());

        // Test with no balls bowled
        assert_eq!(player.overs_bowled(), (0, 0));

        // Test with exactly 1 over
        player.balls_bowled = 6;
        assert_eq!(player.overs_bowled(), (1, 0));

        // Test with 1.3 overs (9 balls)
        player.balls_bowled = 9;
        assert_eq!(player.overs_bowled(), (1, 3));

        // Test with 5 overs exactly
        player.balls_bowled = 30;
        assert_eq!(player.overs_bowled(), (5, 0));

        // Test with 10.5 overs (65 balls)
        player.balls_bowled = 65;
        assert_eq!(player.overs_bowled(), (10, 5));

        // Test with partial over
        player.balls_bowled = 4;
        assert_eq!(player.overs_bowled(), (0, 4));
    }

    #[test]
    fn test_player_display_batting_only() {
        let mut player = Player::new("Pure Batsman".to_string());
        player.runs = 50;
        player.balls_faced = 40;
        player.fours = 5;
        player.sixes = 2;
        player.out = false;

        let display = format!("{}", player);
        assert!(display.contains("Pure Batsman"));
        assert!(display.contains("Batting: 50*(40), 5 4s, 2 6s, SR: 125.00"));
        assert!(!display.contains("Bowling"));
    }

    #[test]
    fn test_player_display_bowling_only() {
        let mut player = Player::new("Pure Bowler".to_string());
        player.balls_bowled = 24;
        player.runs_conceded = 15;
        player.wickets_taken = 2;
        player.maidens = 1;
        player.wides = 2;
        player.no_balls = 1;

        let display = format!("{}", player);
        assert!(display.contains("Pure Bowler"));
        assert!(
            display.contains("Bowling: 2-15 (4 overs), 1 maidens, 2 wides, 1 no balls, Econ: 3.75")
        );
        assert!(!display.contains("Batting"));
    }

    #[test]
    fn test_player_display_all_rounder() {
        let mut player = Player::new("All Rounder".to_string());
        // Batting stats
        player.runs = 35;
        player.balls_faced = 28;
        player.fours = 3;
        player.sixes = 1;
        player.out = true;
        // Bowling stats
        player.balls_bowled = 18;
        player.runs_conceded = 22;
        player.wickets_taken = 1;
        player.maidens = 0;
        player.wides = 1;
        player.no_balls = 0;

        let display = format!("{}", player);
        assert!(display.contains("All Rounder"));
        assert!(display.contains("Batting: 35(28), 3 4s, 1 6s, SR: 125.00"));
        assert!(
            display.contains("Bowling: 1-22 (3 overs), 0 maidens, 1 wides, 0 no balls, Econ: 7.33")
        );
    }

    #[test]
    fn test_player_display_bowling_partial_over() {
        let mut player = Player::new("Medium Pacer".to_string());
        player.balls_bowled = 10; // 1.4 overs
        player.runs_conceded = 8;
        player.wickets_taken = 1;

        let display = format!("{}", player);
        assert!(display.contains("Bowling: 1-8 (1.4 overs)"));
    }

    #[test]
    fn test_player_display_no_stats() {
        let player = Player::new("Substitute".to_string());
        let display = format!("{}", player);
        assert_eq!(display, "Substitute: No stats");
    }

    #[test]
    fn test_bowling_stats_modification() {
        let mut player = Player::new("Test Bowler".to_string());

        // Simulate bowling
        player.balls_bowled += 6;
        player.runs_conceded += 4;
        player.wickets_taken += 1;
        player.maidens += 1;

        assert_eq!(player.balls_bowled, 6);
        assert_eq!(player.runs_conceded, 4);
        assert_eq!(player.wickets_taken, 1);
        assert_eq!(player.maidens, 1);
    }

    #[test]
    fn test_bowling_complex_scenario() {
        let mut player = Player::new("Top Bowler".to_string());

        // Simulate a full bowling spell: 10 overs, 5 wickets, 25 runs, 2 maidens, 3 wides, 1 no ball
        player.balls_bowled = 60;
        player.runs_conceded = 25;
        player.wickets_taken = 5;
        player.maidens = 2;
        player.wides = 3;
        player.no_balls = 1;

        // Check calculations
        assert_eq!(player.overs_bowled(), (10, 0));
        assert_eq!(player.economy_rate(), Some(2.5));
        assert_eq!(player.bowling_average(), Some(5.0));
        assert_eq!(player.bowling_strike_rate(), Some(12.0));

        let display = format!("{}", player);
        assert!(display
            .contains("Bowling: 5-25 (10 overs), 2 maidens, 3 wides, 1 no balls, Econ: 2.50"));
    }

    #[test]
    fn test_economy_rate_edge_cases() {
        let mut player = Player::new("Expensive Bowler".to_string());

        // Very expensive bowling
        player.balls_bowled = 6;
        player.runs_conceded = 30;
        assert_eq!(player.economy_rate(), Some(30.0));

        // Very economical bowling
        player.balls_bowled = 24;
        player.runs_conceded = 4;
        assert_eq!(player.economy_rate(), Some(1.0));

        // Perfect maiden over
        player.balls_bowled = 6;
        player.runs_conceded = 0;
        assert_eq!(player.economy_rate(), Some(0.0));
    }
}

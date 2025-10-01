use core::fmt;
use serde::{Deserialize, Serialize};

/// Represents a cricket player with their batting statistics.
///
/// # Fields
///
/// * `name` - The player's name
/// * `runs` - Total runs scored
/// * `balls_faced` - Number of balls faced
/// * `fours` - Number of boundaries (4 runs)
/// * `sixes` - Number of sixes (6 runs)
/// * `out` - Whether the player is out
/// * `dismissal` - The type of dismissal if the player is out (e.g., "bowled", "caught", "run out")
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub runs: i32,
    pub balls_faced: i32,
    pub fours: i32,
    pub sixes: i32,
    pub out: bool,
    /// The method of dismissal (e.g., "bowled", "caught", "lbw", "run out").
    /// Set to `None` if the player is not out.
    pub dismissal: Option<String>,
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
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut not_out_ind = "";
        if !self.out {
            not_out_ind = "*";
        }

        let strike_rate_str = match self.strike_rate() {
            Some(sr) => format!(", SR: {:.2}", sr),
            None => String::new(),
        };

        write!(
            f,
            "{}: {}{}({}), {} 4s, {} 6s{}",
            self.name,
            self.runs,
            not_out_ind,
            self.balls_faced,
            self.fours,
            self.sixes,
            strike_rate_str
        )
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
        assert_eq!(display, "Bob Wilson: 45*(30), 5 4s, 1 6s, SR: 150.00");
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
        assert_eq!(display, "Alice Brown: 25(20), 3 4s, 0 6s, SR: 125.00");
    }

    #[test]
    fn test_player_display_zero_stats() {
        let player = Player::new("New Player".to_string());
        let display = format!("{}", player);
        assert_eq!(display, "New Player: 0*(0), 0 4s, 0 6s");
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
        assert_eq!(display, "Heavy Scorer: 125(85), 12 4s, 4 6s, SR: 147.06");
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
        assert_eq!(display, "Test Batsman: 50*(50), 0 4s, 0 6s, SR: 100.00");

        // Test with decimal strike rate
        player.runs = 33;
        player.balls_faced = 25;
        let display = format!("{}", player);
        assert_eq!(display, "Test Batsman: 33*(25), 0 4s, 0 6s, SR: 132.00");

        // Test with repeating decimal
        player.runs = 10;
        player.balls_faced = 3;
        let display = format!("{}", player);
        assert_eq!(display, "Test Batsman: 10*(3), 0 4s, 0 6s, SR: 333.33");
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
}

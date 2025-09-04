use serde::{Deserialize, Serialize};
use std::collections::{hash_map::Entry, HashMap};

use super::{innings::Innings, player::Team};

/// A complete cricket match with teams, innings, and result calculation.
///
/// The `Match` struct is the central component of the cricket scoring library,
/// representing a complete cricket match from start to finish. It supports all
/// major match formats (Test, ODI, T20, T10) and provides automatic result calculation
/// based on innings data.
///
/// # Key Features
///
/// - **Match Management**: Track teams, venue, date, and match type
/// - **Innings Tracking**: Add multiple innings as the match progresses  
/// - **Automatic Result Calculation**: Use `calculate_result()` to determine winner and margin
/// - **Serialization**: Full serde support for JSON persistence
/// - **Comprehensive Statistics**: Team totals, match status, and victory detection
///
/// # Example
///
/// ```
/// use cricket_scoring::{Match, MatchType, Team, Player, Innings, MatchResult, WinMargin};
///
/// // Create teams
/// let team1 = Team {
///     name: "Australia".to_string(),
///     players: vec![Player::new("Steve Smith".to_string())],
/// };
/// let team2 = Team {
///     name: "England".to_string(),
///     players: vec![Player::new("Joe Root".to_string())],
/// };
///
/// // Create match
/// let mut match_instance = Match::new(
///     "AUS_ENG_2025".to_string(),
///     "Australia vs England".to_string(),
///     MatchType::OD,
///     team1.clone(),
///     team2.clone(),
/// )
/// .with_venue("Melbourne Cricket Ground".to_string())
/// .with_date("2025-01-15".to_string());
///
/// // Add innings data
/// let mut innings1 = Innings::new(team1.clone(), team2.clone());
/// innings1.score.runs = 280;
/// innings1.score.wickets_left = 2;
/// match_instance.add_innings(innings1);
///
/// let mut innings2 = Innings::new(team2, team1);
/// innings2.score.runs = 250;
/// innings2.score.wickets_left = 0; // All out
/// match_instance.add_innings(innings2);
///
/// // Calculate the result
/// match_instance.calculate_result();
///
/// // Check the result
/// match match_instance.result {
///     Some(MatchResult::Team1Won { margin: WinMargin::Runs(runs), method }) => {
///         let method_text = method.map(|m| format!(" {}", m)).unwrap_or_default();
///         println!("Australia won by {} runs{}", runs, method_text); // Australia won by 30 runs
///     }
///     _ => println!("Unexpected result"),
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub id: String,
    pub title: String,
    pub venue: Option<String>,
    pub date: Option<String>,
    pub match_type: MatchType,
    pub team1: Team,
    pub team2: Team,
    pub innings: Vec<Innings>,
    /// Current status of the match
    pub status: MatchStatus,
    /// Final result of the match if completed
    pub result: Option<MatchResult>,
}

/// Types of cricket matches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchType {
    /// Test cricket
    Test,
    /// One Day
    OD,
    /// Twenty20
    T20,
    /// Other match formats with custom description
    Other(String),
}

/// Current status of a cricket match
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum MatchStatus {
    #[default]
    NotStarted,
    InProgress,
    Completed,
    Abandoned,
    NoResult,
}

/// Final result of a completed match
/// Method, if it exists, gives a method for the result (e.g. D/L)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchResult {
    Team1Won { 
        margin: WinMargin, 
        method: Option<String> 
    },
    Team2Won { 
        margin: WinMargin, 
        method: Option<String> 
    },
    Tie { 
        method: Option<String> 
    },
    Draw,
    NoResult,
}

/// Margin of victory in a cricket match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WinMargin {
    /// Victory by a certain number of runs
    Runs(u32),
    /// Victory by a certain number of wickets
    Wickets(u8),
    /// Victory by award (forfeit, disqualification, etc.) - no playing margin
    Award,
}

impl Match {
    /// Shorthand to create a new match with the given details (not all fields)
    #[must_use]
    pub fn new(id: String, title: String, match_type: MatchType, team1: Team, team2: Team) -> Self {
        Match {
            id,
            title,
            venue: None,
            date: None,
            match_type,
            team1,
            team2,
            innings: Vec::new(),
            status: MatchStatus::NotStarted,
            result: None,
        }
    }

    /// Sets the venue for the match
    pub fn with_venue(mut self, venue: String) -> Self {
        self.venue = Some(venue);
        self
    }

    /// Sets the date for the match
    pub fn with_date(mut self, date: String) -> Self {
        self.date = Some(date);
        self
    }

    /// Adds an innings to the match
    pub fn add_innings(&mut self, innings: Innings) {
        self.innings.push(innings);
    }

    /// Sets the match status
    pub fn set_status(&mut self, status: MatchStatus) {
        self.status = status;
    }

    /// Sets the match result
    pub fn set_result(&mut self, result: MatchResult) {
        self.result = Some(result);
        self.status = MatchStatus::Completed;
    }

    /// Sets the match result with method information (e.g. "D/L", "VJD", etc.)
    pub fn set_result_with_method(&mut self, result: MatchResult, method: Option<String>) {
        let result_with_method = match result {
            MatchResult::Team1Won { margin, .. } => MatchResult::Team1Won { margin, method },
            MatchResult::Team2Won { margin, .. } => MatchResult::Team2Won { margin, method },
            MatchResult::Tie { .. } => MatchResult::Tie { method },
            other => other, // Draw and NoResult don't have method fields
        };
        self.set_result(result_with_method);
    }

    /// Returns true if the match is completed
    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(self.status, MatchStatus::Completed)
    }

    /// Returns true if the match is in progress
    #[must_use]
    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, MatchStatus::InProgress)
    }

    /// Gets the total runs scored by team1 across all their innings
    #[must_use]
    pub fn team1_total_runs(&self) -> i32 {
        self.innings
            .iter()
            .filter(|innings| innings.batting_team.name == self.team1.name)
            .map(|innings| innings.score.runs)
            .sum()
    }

    /// Gets the total runs scored by team2 across all their innings
    #[must_use]
    pub fn team2_total_runs(&self) -> i32 {
        self.innings
            .iter()
            .filter(|innings| innings.batting_team.name == self.team2.name)
            .map(|innings| innings.score.runs)
            .sum()
    }

    /// Calculate the match result based on innings data.
    ///
    /// This method analyzes all completed innings to determine the match winner and margin of victory.
    /// It handles various match scenarios including:
    ///
    /// - **Wins by runs**: When the team batting first scores more runs
    /// - **Wins by wickets**: When the team batting second reaches the target with wickets remaining
    /// - **Ties**: When both teams score exactly the same number of runs
    /// - **Draws**: When the match is incomplete or the chasing team didn't reach the target but had wickets remaining
    ///
    /// The calculation automatically sets both `status` to `Completed` and populates the `result` field.
    ///
    /// # Example
    ///
    /// ```
    /// use cricket_scoring::{Match, MatchType, Team, Player, Innings, MatchResult};
    ///
    /// let team1 = Team { name: "Team A".to_string(), players: vec![] };
    /// let team2 = Team { name: "Team B".to_string(), players: vec![] };
    /// let mut cricket_match = Match::new("M001".to_string(), "Test".to_string(),
    ///                                   MatchType::OD, team1.clone(), team2.clone());
    ///
    /// // Add innings (Team A: 200, Team B: 150 all out)
    /// let mut innings1 = Innings::new(team1.clone(), team2.clone());
    /// innings1.score.runs = 200;
    /// cricket_match.add_innings(innings1);
    ///
    /// let mut innings2 = Innings::new(team2, team1);
    /// innings2.score.runs = 150;
    /// innings2.score.wickets_left = 0; // All out
    /// cricket_match.add_innings(innings2);
    ///
    /// // Calculate result
    /// cricket_match.calculate_result();
    ///
    /// assert!(cricket_match.is_completed());
    /// // Result will be Team1Won by 50 runs
    /// ```
    pub fn calculate_result(&mut self) {
        if self.innings.is_empty() {
            return;
        }

        let mut scores: HashMap<String, Vec<i32>> = HashMap::new();
        let mut teams: Vec<String> = vec![];
        let mut bowling_team = String::new();
        let mut batting_team = String::new();
        let mut last_innings_wickets_left: Option<i32> = None;

        for innings in &self.innings {
            let team_name = innings.batting_team.name.clone();
            batting_team = team_name.clone();
            bowling_team = innings.bowling_team.name.clone();
            teams.push(team_name.clone());
            if let Entry::Vacant(e) = scores.entry(team_name.clone()) {
                e.insert(vec![innings.score.runs]);
            } else {
                scores
                    .get_mut(&team_name.clone())
                    .unwrap()
                    .push(innings.score.runs);
            };
            last_innings_wickets_left = Some(innings.score.wickets_left);
        }

        // 0 or 1 innings - game not complete
        let not_finished = scores.len() < 2;
        // last team didn't score enough runs, but had wickets left
        let is_draw = scores
            .get(&batting_team)
            .unwrap_or(&vec![])
            .iter()
            .sum::<i32>()
            < scores
                .get(&bowling_team)
                .unwrap_or(&vec![])
                .iter()
                .sum::<i32>()
            && last_innings_wickets_left.unwrap_or(0) > 0;

        if not_finished || is_draw {
            self.result = Some(MatchResult::Draw);
            self.status = MatchStatus::Completed;
            return;
        }

        let team_a = teams[0].clone();
        let team_b = teams[1].clone();
        let team_a_total: i32 = scores.get(&team_a).unwrap_or(&vec![]).iter().sum();
        let team_b_total: i32 = scores.get(&team_b).unwrap_or(&vec![]).iter().sum();

        self.result = match team_a_total.cmp(&team_b_total) {
            std::cmp::Ordering::Greater => {
                let margin = self.calculate_win_margin(
                    &team_a,
                    &team_b,
                    &batting_team,
                    &scores,
                    last_innings_wickets_left.unwrap_or(0),
                );
                if team_a == self.team1.name {
                    Some(MatchResult::Team1Won { margin, method: None })
                } else {
                    Some(MatchResult::Team2Won { margin, method: None })
                }
            }
            std::cmp::Ordering::Equal => Some(MatchResult::Tie { method: None }),
            std::cmp::Ordering::Less => {
                let margin = self.calculate_win_margin(
                    &team_b,
                    &team_a,
                    &batting_team,
                    &scores,
                    last_innings_wickets_left.unwrap_or(0),
                );
                if team_b == self.team1.name {
                    Some(MatchResult::Team1Won { margin, method: None })
                } else {
                    Some(MatchResult::Team2Won { margin, method: None })
                }
            }
        };

        self.status = MatchStatus::Completed;
    }

    /// Calculate the margin of victory
    fn calculate_win_margin(
        &self,
        winning_team: &str,
        losing_team: &str,
        batting_team: &str,
        scores: &HashMap<String, Vec<i32>>,
        last_innings_wickets_left: i32,
    ) -> WinMargin {
        // For innings victories, the margin is always in runs
        if self.is_innings_victory() {
            let winning_score: i32 = scores.get(winning_team).unwrap().iter().sum();
            let losing_score: i32 = scores.get(losing_team).unwrap().iter().sum();
            WinMargin::Runs((winning_score - losing_score) as u32)
        } else if winning_team == batting_team {
            // Team batting last won - victory by wickets
            WinMargin::Wickets(last_innings_wickets_left as u8)
        } else {
            // Team batting first won - victory by runs
            let winning_score: i32 = scores.get(winning_team).unwrap().iter().sum();
            let losing_score: i32 = scores.get(losing_team).unwrap().iter().sum();
            WinMargin::Runs((winning_score - losing_score) as u32)
        }
    }

    /// Check if this is an innings victory (team won without needing all their innings)
    #[must_use]
    pub fn is_innings_victory(&self) -> bool {
        if self.innings.len() < 3 {
            return false;
        }

        let mut team_innings_count: HashMap<String, usize> = HashMap::new();
        for innings in &self.innings {
            *team_innings_count
                .entry(innings.batting_team.name.clone())
                .or_insert(0) += 1;
        }

        // One team has fewer completed innings than the other
        let counts: Vec<usize> = team_innings_count.values().cloned().collect();
        counts.len() == 2 && counts[0] != counts[1]
    }
}

impl Default for MatchType {
    fn default() -> Self {
        MatchType::Other("Unknown".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scoring::player::Player;

    fn create_test_team(name: &str) -> Team {
        Team {
            name: name.to_string(),
            players: vec![
                Player::new("Player1".to_string()),
                Player::new("Player2".to_string()),
            ],
        }
    }

    fn create_test_innings(batting_team: Team, bowling_team: Team, runs: i32) -> Innings {
        let mut innings = Innings::new(batting_team, bowling_team);
        innings.score.runs = runs;
        innings
    }

    #[test]
    fn test_match_new() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");

        let match_instance = Match::new(
            "M001".to_string(),
            "Test Match".to_string(),
            MatchType::Test,
            team1.clone(),
            team2.clone(),
        );

        assert_eq!(match_instance.id, "M001");
        assert_eq!(match_instance.title, "Test Match");
        assert!(match_instance.venue.is_none());
        assert!(match_instance.date.is_none());
        assert!(matches!(match_instance.match_type, MatchType::Test));
        assert_eq!(match_instance.team1.name, team1.name);
        assert_eq!(match_instance.team2.name, team2.name);
        assert!(match_instance.innings.is_empty());
        assert!(matches!(match_instance.status, MatchStatus::NotStarted));
        assert!(match_instance.result.is_none());
    }

    #[test]
    fn test_match_with_venue_and_date() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");

        let match_instance = Match::new(
            "M002".to_string(),
            "ODI Match".to_string(),
            MatchType::OD,
            team1,
            team2,
        )
        .with_venue("Lord's Cricket Ground".to_string())
        .with_date("2025-01-01".to_string());

        assert_eq!(
            match_instance.venue,
            Some("Lord's Cricket Ground".to_string())
        );
        assert_eq!(match_instance.date, Some("2025-01-01".to_string()));
    }

    #[test]
    fn test_add_innings() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M003".to_string(),
            "T20 Match".to_string(),
            MatchType::T20,
            team1.clone(),
            team2.clone(),
        );

        let innings1 = create_test_innings(team1.clone(), team2.clone(), 150);
        let innings2 = create_test_innings(team2, team1, 145);

        match_instance.add_innings(innings1);
        match_instance.add_innings(innings2);

        assert_eq!(match_instance.innings.len(), 2);
        assert_eq!(match_instance.innings[0].score.runs, 150);
        assert_eq!(match_instance.innings[1].score.runs, 145);
    }

    #[test]
    fn test_set_status() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M004".to_string(),
            "Test Match".to_string(),
            MatchType::Test,
            team1,
            team2,
        );

        match_instance.set_status(MatchStatus::InProgress);
        assert!(matches!(match_instance.status, MatchStatus::InProgress));

        match_instance.set_status(MatchStatus::Completed);
        assert!(matches!(match_instance.status, MatchStatus::Completed));
    }

    #[test]
    fn test_set_result() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M005".to_string(),
            "ODI Match".to_string(),
            MatchType::OD,
            team1,
            team2,
        );

        let result = MatchResult::Team1Won {
            margin: WinMargin::Runs(25),
            method: None,
        };
        match_instance.set_result(result);

        assert!(match_instance.result.is_some());
        assert!(matches!(match_instance.status, MatchStatus::Completed));
        match match_instance.result.unwrap() {
            MatchResult::Team1Won {
                margin: WinMargin::Runs(runs),
                ..
            } => assert_eq!(runs, 25),
            _ => panic!("Expected Team1Won with Runs margin"),
        }
    }

    #[test]
    fn test_match_status_queries() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M006".to_string(),
            "T20 Match".to_string(),
            MatchType::T20,
            team1,
            team2,
        );

        assert!(!match_instance.is_completed());
        assert!(!match_instance.is_in_progress());

        match_instance.set_status(MatchStatus::InProgress);
        assert!(match_instance.is_in_progress());
        assert!(!match_instance.is_completed());

        match_instance.set_status(MatchStatus::Completed);
        assert!(match_instance.is_completed());
        assert!(!match_instance.is_in_progress());
    }

    #[test]
    fn test_team_total_runs() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M007".to_string(),
            "Test Match".to_string(),
            MatchType::Test,
            team1.clone(),
            team2.clone(),
        );

        // Team A first innings: 250 runs
        let innings1 = create_test_innings(team1.clone(), team2.clone(), 250);
        match_instance.add_innings(innings1);

        // Team B first innings: 200 runs
        let innings2 = create_test_innings(team2.clone(), team1.clone(), 200);
        match_instance.add_innings(innings2);

        // Team A second innings: 150 runs
        let innings3 = create_test_innings(team1.clone(), team2.clone(), 150);
        match_instance.add_innings(innings3);

        // Team B second innings: 180 runs
        let innings4 = create_test_innings(team2, team1, 180);
        match_instance.add_innings(innings4);

        assert_eq!(match_instance.team1_total_runs(), 400); // 250 + 150
        assert_eq!(match_instance.team2_total_runs(), 380); // 200 + 180
    }

    #[test]
    fn test_match_type_variants() {
        assert!(matches!(MatchType::Test, MatchType::Test));
        assert!(matches!(MatchType::OD, MatchType::OD));
        assert!(matches!(MatchType::T20, MatchType::T20));

        let custom = MatchType::Other("The Hundred".to_string());
        match custom {
            MatchType::Other(ref name) => assert_eq!(name, "The Hundred"),
            _ => panic!("Expected Other variant"),
        }
    }

    #[test]
    fn test_win_margin_variants() {
        let runs_margin = WinMargin::Runs(42);
        match runs_margin {
            WinMargin::Runs(runs) => assert_eq!(runs, 42),
            _ => panic!("Expected Runs variant"),
        }

        let wickets_margin = WinMargin::Wickets(5);
        match wickets_margin {
            WinMargin::Wickets(wickets) => assert_eq!(wickets, 5),
            _ => panic!("Expected Wickets variant"),
        }

        let award_margin = WinMargin::Award;
        assert!(matches!(award_margin, WinMargin::Award));
    }

    #[test]
    fn test_awarded_match() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M_AWARD".to_string(),
            "Awarded Match".to_string(),
            MatchType::OD,
            team1,
            team2,
        );

        // Set an awarded result (e.g., due to forfeit)
        let result = MatchResult::Team1Won {
            margin: WinMargin::Award,
            method: Some("forfeit".to_string()),
        };
        match_instance.set_result(result);

        assert!(match_instance.is_completed());
        match match_instance.result.unwrap() {
            MatchResult::Team1Won {
                margin: WinMargin::Award,
                method: Some(method_str),
            } => {
                assert_eq!(method_str, "forfeit");
            }
            _ => panic!("Expected Team1Won by award"),
        }
    }

    #[test]
    fn test_match_result_variants() {
        let team1_won = MatchResult::Team1Won {
            margin: WinMargin::Runs(30),
            method: None,
        };
        assert!(matches!(team1_won, MatchResult::Team1Won { .. }));

        let team2_won = MatchResult::Team2Won {
            margin: WinMargin::Wickets(7),
            method: None,
        };
        assert!(matches!(team2_won, MatchResult::Team2Won { .. }));

        assert!(matches!(MatchResult::Tie { method: None }, MatchResult::Tie { .. }));
        assert!(matches!(MatchResult::Draw, MatchResult::Draw));
        assert!(matches!(MatchResult::NoResult, MatchResult::NoResult));
    }

    #[test]
    fn test_defaults() {
        let default_match_type = MatchType::default();
        match default_match_type {
            MatchType::Other(ref name) => assert_eq!(name, "Unknown"),
            _ => panic!("Expected Other variant with 'Unknown'"),
        }

        let default_status = MatchStatus::default();
        assert!(matches!(default_status, MatchStatus::NotStarted));
    }

    #[test]
    fn test_match_clone() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let match_instance = Match::new(
            "M008".to_string(),
            "Clone Test".to_string(),
            MatchType::T20,
            team1,
            team2,
        );

        let cloned_match = match_instance.clone();
        assert_eq!(match_instance.id, cloned_match.id);
        assert_eq!(match_instance.title, cloned_match.title);
        assert_eq!(match_instance.team1.name, cloned_match.team1.name);
        assert_eq!(match_instance.team2.name, cloned_match.team2.name);
    }

    #[test]
    fn test_calculate_result_simple_win_by_runs() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M009".to_string(),
            "Test Match".to_string(),
            MatchType::OD,
            team1.clone(),
            team2.clone(),
        );

        // Team A scores 200
        let innings1 = create_test_innings(team1.clone(), team2.clone(), 200);
        match_instance.add_innings(innings1);

        // Team B scores 150 (all out)
        let mut innings2 = create_test_innings(team2, team1, 150);
        innings2.score.wickets_left = 0;
        match_instance.add_innings(innings2);

        match_instance.calculate_result();

        assert!(matches!(match_instance.status, MatchStatus::Completed));
        match match_instance.result.unwrap() {
            MatchResult::Team1Won {
                margin: WinMargin::Runs(runs),
                ..
            } => assert_eq!(runs, 50),
            _ => panic!("Expected Team1Won by runs"),
        }
    }

    #[test]
    fn test_calculate_result_win_by_wickets() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M010".to_string(),
            "Test Match".to_string(),
            MatchType::OD,
            team1.clone(),
            team2.clone(),
        );

        // Team A scores 150 (all out)
        let mut innings1 = create_test_innings(team1.clone(), team2.clone(), 150);
        innings1.score.wickets_left = 0;
        match_instance.add_innings(innings1);

        // Team B scores 151 with 4 wickets left
        let mut innings2 = create_test_innings(team2, team1, 151);
        innings2.score.wickets_left = 4;
        match_instance.add_innings(innings2);

        match_instance.calculate_result();

        assert!(matches!(match_instance.status, MatchStatus::Completed));
        match match_instance.result.unwrap() {
            MatchResult::Team2Won {
                margin: WinMargin::Wickets(wickets),
                ..
            } => assert_eq!(wickets, 4),
            _ => panic!("Expected Team2Won by wickets"),
        }
    }

    #[test]
    fn test_calculate_result_tie() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M011".to_string(),
            "Test Match".to_string(),
            MatchType::T20,
            team1.clone(),
            team2.clone(),
        );

        // Both teams score 180
        let innings1 = create_test_innings(team1.clone(), team2.clone(), 180);
        let innings2 = create_test_innings(team2, team1, 180);
        match_instance.add_innings(innings1);
        match_instance.add_innings(innings2);

        match_instance.calculate_result();

        assert!(matches!(match_instance.status, MatchStatus::Completed));
        assert!(matches!(match_instance.result.unwrap(), MatchResult::Tie { .. }));
    }

    #[test]
    fn test_calculate_result_draw() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M012".to_string(),
            "Test Match".to_string(),
            MatchType::Test,
            team1.clone(),
            team2.clone(),
        );

        // Team A scores 300
        let innings1 = create_test_innings(team1.clone(), team2.clone(), 300);
        match_instance.add_innings(innings1);

        // Team B scores 200 with wickets left (didn't reach target)
        let mut innings2 = create_test_innings(team2, team1, 200);
        innings2.score.wickets_left = 5;
        match_instance.add_innings(innings2);

        match_instance.calculate_result();

        assert!(matches!(match_instance.status, MatchStatus::Completed));
        assert!(matches!(match_instance.result.unwrap(), MatchResult::Draw));
    }

    #[test]
    fn test_set_result_with_method() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M_METHOD".to_string(),
            "Method Test".to_string(),
            MatchType::OD,
            team1,
            team2,
        );

        // Test setting result with Duckworth-Lewis method
        let result = MatchResult::Team1Won {
            margin: WinMargin::Runs(15),
            method: Some("D/L".to_string()),
        };
        match_instance.set_result_with_method(result, Some("D/L".to_string()));

        assert!(match_instance.is_completed());
        match match_instance.result.unwrap() {
            MatchResult::Team1Won {
                margin: WinMargin::Runs(runs),
                method: Some(method_str),
            } => {
                assert_eq!(runs, 15);
                assert_eq!(method_str, "D/L");
            }
            _ => panic!("Expected Team1Won with method"),
        }
    }

    #[test]
    fn test_match_result_with_method_serialization() {
        let result_with_method = MatchResult::Team2Won {
            margin: WinMargin::Wickets(3),
            method: Some("VJD".to_string()),
        };

        let json = serde_json::to_string(&result_with_method).unwrap();
        let deserialized: MatchResult = serde_json::from_str(&json).unwrap();

        match deserialized {
            MatchResult::Team2Won {
                margin: WinMargin::Wickets(wickets),
                method: Some(method_str),
            } => {
                assert_eq!(wickets, 3);
                assert_eq!(method_str, "VJD");
            }
            _ => panic!("Expected Team2Won with method"),
        }
    }

    #[test]
    fn test_is_innings_victory() {
        let team1 = create_test_team("Team A");
        let team2 = create_test_team("Team B");
        let mut match_instance = Match::new(
            "M013".to_string(),
            "Test Match".to_string(),
            MatchType::Test,
            team1.clone(),
            team2.clone(),
        );

        // Team A first innings: 400
        let innings1 = create_test_innings(team1.clone(), team2.clone(), 400);
        match_instance.add_innings(innings1);

        // Team B first innings: 150
        let innings2 = create_test_innings(team2.clone(), team1.clone(), 150);
        match_instance.add_innings(innings2);

        // Team B second innings: 200 (still behind by 50)
        let innings3 = create_test_innings(team2, team1, 200);
        match_instance.add_innings(innings3);

        assert!(match_instance.is_innings_victory());
    }
}

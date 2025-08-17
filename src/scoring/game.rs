use std::collections::{hash_map::Entry, HashMap};

use super::{innings::Innings, player::Team};

#[derive(Debug)]
pub struct Game {
    pub teams: HashMap<String, Team>,
    pub innings: Vec<Innings>,
    pub meta: Meta,
    pub outcome: Option<Outcome>,
}

#[derive(Default, Debug)]
pub struct Outcome {
    pub draw: bool,
    pub tie: bool,
    pub winner: Option<String>,
    pub runs_margin: Option<i32>,
    pub wickets_margin: Option<i32>,
    pub method: Option<String>,
    pub innings_win: bool,
    pub result: bool,
}

#[derive(Debug)]
pub struct Meta {
    pub venue: Option<String>,
}

fn get_margin(
    winning_team: &str,
    losing_team: &str,
    batting_team: &str,
    scores: &HashMap<String, Vec<i32>>,
    last_innings_wickets_lost: i32,
) -> Outcome {
    let mut runs_margin = None;
    let mut wickets_margin = None;
    let innings_win =
        scores.get(winning_team).unwrap().len() < scores.get(losing_team).unwrap().len();
    if winning_team == batting_team {
        wickets_margin = Some(last_innings_wickets_lost);
    } else {
        let winning_score: i32 = scores.get(winning_team).unwrap().iter().sum();
        let losing_score: i32 = scores.get(losing_team).unwrap().iter().sum();
        runs_margin = Some(winning_score - losing_score);
    };
    Outcome {
        draw: false,
        tie: false,
        winner: Some(winning_team.to_string()),
        runs_margin,
        wickets_margin,
        method: None,
        innings_win,
        result: true,
    }
}

impl Game {
    pub fn new(meta: Meta, teams: HashMap<String, Team>) -> Self {
        Game {
            meta,
            teams,
            innings: vec![],
            outcome: None,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn score(&mut self, outcome: Outcome) {
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

        if outcome.method.is_some() {
            self.outcome = Some(outcome);
            return;
        }

        if !outcome.result {
            self.outcome = Some(outcome);
            return;
        }

        // 0 or 1 innings
        let not_finished = scores.len() < 2;
        // last team didn't score enough runs, but had wickets left (NOTE: Doesn't check whether
        // they ran out of of time)
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
            && last_innings_wickets_left.unwrap() > 0
            && outcome.winner.is_none();
        // let no_result = winner
        if not_finished || is_draw {
            self.outcome = Some(Outcome {
                draw: true,
                tie: false,
                winner: None,
                runs_margin: None,
                wickets_margin: None,
                method: None,
                innings_win: false,
                result: true,
            });
            return;
        }
        let team_a = teams[0].clone();
        let team_b = teams[1].clone();

        self.outcome = match scores
            .get(&team_a)
            .unwrap_or(&vec![])
            .iter()
            .sum::<i32>()
            .cmp(&scores.get(&team_b).unwrap_or(&vec![]).iter().sum::<i32>())
        {
            std::cmp::Ordering::Greater => Some(get_margin(
                &team_a,
                &team_b,
                &batting_team,
                &scores,
                last_innings_wickets_left.unwrap(),
            )),
            std::cmp::Ordering::Equal => Some(Outcome {
                tie: true,
                ..Default::default()
            }),
            std::cmp::Ordering::Less => Some(get_margin(
                &team_b,
                &team_a,
                &batting_team,
                &scores,
                last_innings_wickets_left.unwrap(),
            )),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scoring::{innings::Innings, player::Player};

    fn create_test_team(name: &str) -> Team {
        Team {
            name: name.to_string(),
            players: vec![
                Player::new("Player1".to_string()),
                Player::new("Player2".to_string()),
            ],
        }
    }

    fn create_test_game() -> Game {
        let team_a = create_test_team("TeamA");
        let team_b = create_test_team("TeamB");
        let mut teams = HashMap::new();
        teams.insert("TeamA".to_string(), team_a);
        teams.insert("TeamB".to_string(), team_b);

        Game::new(
            Meta {
                venue: Some("Test Ground".to_string()),
            },
            teams,
        )
    }

    fn create_test_innings(
        batting_team_name: &str,
        bowling_team_name: &str,
        runs: i32,
        wickets_left: i32,
    ) -> Innings {
        let batting_team = create_test_team(batting_team_name);
        let bowling_team = create_test_team(bowling_team_name);
        let mut innings = Innings::new(batting_team, bowling_team);
        innings.score.runs = runs;
        innings.score.wickets_left = wickets_left;
        innings.score.wickets_lost = 10 - wickets_left;
        innings
    }

    #[test]
    fn test_simple_win_by_runs() {
        let mut game = create_test_game();

        // TeamA scores 100
        game.innings
            .push(create_test_innings("TeamA", "TeamB", 100, 8));
        // TeamB scores 80 (all out)
        game.innings
            .push(create_test_innings("TeamB", "TeamA", 80, 0));

        let outcome = Outcome {
            result: true,
            ..Default::default()
        };

        game.score(outcome);

        let result = game.outcome.unwrap();
        assert!(!result.draw);
        assert!(!result.tie);
        assert_eq!(result.winner, Some("TeamA".to_string()));
        assert_eq!(result.runs_margin, Some(20));
        assert!(result.wickets_margin.is_none());
        assert!(!result.innings_win);
    }

    #[test]
    fn test_simple_win_by_wickets() {
        let mut game = create_test_game();

        // TeamA scores 100 (all out)
        game.innings
            .push(create_test_innings("TeamA", "TeamB", 100, 0));
        // TeamB scores 101 with 6 wickets left
        game.innings
            .push(create_test_innings("TeamB", "TeamA", 101, 6));

        let outcome = Outcome {
            result: true,
            ..Default::default()
        };

        game.score(outcome);

        let result = game.outcome.unwrap();
        assert!(!result.draw);
        assert!(!result.tie);
        assert_eq!(result.winner, Some("TeamB".to_string()));
        assert!(result.runs_margin.is_none());
        assert_eq!(result.wickets_margin, Some(6));
        assert!(!result.innings_win);
    }

    #[test]
    fn test_tie() {
        let mut game = create_test_game();

        // Both teams score 150
        game.innings
            .push(create_test_innings("TeamA", "TeamB", 150, 5));
        game.innings
            .push(create_test_innings("TeamB", "TeamA", 150, 3));

        let outcome = Outcome {
            result: true,
            ..Default::default()
        };

        game.score(outcome);

        let result = game.outcome.unwrap();
        assert!(!result.draw);
        assert!(result.tie);
        assert!(result.winner.is_none());
        assert!(result.runs_margin.is_none());
        assert!(result.wickets_margin.is_none());
    }

    #[test]
    fn test_draw() {
        let mut game = create_test_game();

        // TeamA scores 200
        game.innings
            .push(create_test_innings("TeamA", "TeamB", 200, 2));
        // TeamB scores 150 with wickets left (didn't reach target)
        game.innings
            .push(create_test_innings("TeamB", "TeamA", 150, 4));

        let outcome = Outcome {
            result: true,
            winner: None,
            ..Default::default()
        };

        game.score(outcome);

        let result = game.outcome.unwrap();
        assert!(result.draw);
        assert!(!result.tie);
        assert!(result.winner.is_none());
    }

    #[test]
    fn test_innings_win() {
        let mut game = create_test_game();

        // TeamA scores 400
        game.innings
            .push(create_test_innings("TeamA", "TeamB", 400, 3));
        // TeamB scores 150 (all out)
        game.innings
            .push(create_test_innings("TeamB", "TeamA", 150, 0));
        // TeamB scores 200 in follow-on (all out) - TeamA still ahead
        game.innings
            .push(create_test_innings("TeamB", "TeamA", 200, 0));

        let outcome = Outcome {
            result: true,
            ..Default::default()
        };

        game.score(outcome);

        let result = game.outcome.unwrap();
        assert!(!result.draw);
        assert!(!result.tie);
        assert_eq!(result.winner, Some("TeamA".to_string()));
        assert_eq!(result.runs_margin, Some(50));
        assert!(result.innings_win);
    }

    #[test]
    fn test_no_result_method() {
        let mut game = create_test_game();
        game.innings
            .push(create_test_innings("TeamA", "TeamB", 100, 5));

        let outcome = Outcome {
            method: Some("rain".to_string()),
            result: false,
            ..Default::default()
        };

        game.score(outcome);

        let result = game.outcome.unwrap();
        assert_eq!(result.method, Some("rain".to_string()));
        assert!(!result.result);
    }

    #[test]
    fn test_no_result_without_method() {
        let mut game = create_test_game();
        game.innings
            .push(create_test_innings("TeamA", "TeamB", 100, 5));

        let outcome = Outcome {
            result: false,
            ..Default::default()
        };

        game.score(outcome);

        let result = game.outcome.unwrap();
        assert!(!result.result);
        assert!(result.method.is_none());
    }

    #[test]
    fn test_not_finished_game() {
        let mut game = create_test_game();
        // Only one team has batted
        game.innings
            .push(create_test_innings("TeamA", "TeamB", 200, 3));

        let outcome = Outcome {
            result: true,
            ..Default::default()
        };

        game.score(outcome);

        let result = game.outcome.unwrap();
        assert!(result.draw);
        assert!(!result.tie);
        assert!(result.winner.is_none());
    }

    #[test]
    fn test_multiple_innings_per_team() {
        let mut game = create_test_game();

        // TeamA first innings: 200
        game.innings
            .push(create_test_innings("TeamA", "TeamB", 200, 2));
        // TeamB first innings: 150
        game.innings
            .push(create_test_innings("TeamB", "TeamA", 150, 0));
        // TeamA second innings: 100
        game.innings
            .push(create_test_innings("TeamA", "TeamB", 100, 5));
        // TeamB second innings: 120 (chasing 251)
        game.innings
            .push(create_test_innings("TeamB", "TeamA", 120, 0));

        let outcome = Outcome {
            result: true,
            ..Default::default()
        };

        game.score(outcome);

        let result = game.outcome.unwrap();
        assert!(!result.draw);
        assert!(!result.tie);
        assert_eq!(result.winner, Some("TeamA".to_string()));
        // TeamA total: 300, TeamB total: 270, margin: 30
        assert_eq!(result.runs_margin, Some(30));
    }
}

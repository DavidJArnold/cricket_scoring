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
            batting_team.clone_from(&team_name);
            bowling_team.clone_from(&innings.bowling_team.name);
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

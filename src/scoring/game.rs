use std::collections::HashMap;

use super::{innings::Innings, player::Team};

pub struct Game {
    pub teams: HashMap<String, Team>,
    pub innings: Vec<Innings>,
    pub meta: GameMeta,
    pub outcome: Option<Outcome>,
}

#[derive(Default)]
pub struct Outcome {
    pub draw: bool,
    pub tie: bool,
    pub winner: Option<String>,
    pub runs_margin: Option<i32>,
    pub wickets_margin: Option<i32>,
}

pub struct GameMeta {
    pub venue: Option<String>,
}

impl Game {
    fn get_margin(
        &self,
        winning_team: String,
        losing_team: String,
        batting_team: String,
        scores: HashMap<String, i32>,
    ) -> Outcome {
        let mut runs_margin = None;
        let mut wickets_margin = None;
        if winning_team == batting_team {
            wickets_margin = Some(1);
        } else {
            runs_margin =
                Some(scores.get(&winning_team).unwrap() - scores.get(&losing_team).unwrap());
        };
        Outcome {
            draw: false,
            tie: false,
            winner: Some(winning_team),
            runs_margin,
            wickets_margin,
        }
    }
    pub fn score(&mut self) {
        let mut scores: HashMap<String, i32> = HashMap::new();
        let mut teams: Vec<String> = vec![];
        let mut batting_team = String::new();
        for innings in self.innings.iter() {
            let team_name = innings.batting_team.name.clone();
            batting_team = team_name.clone();
            teams.push(team_name.clone());
            if scores.contains_key(&team_name) {
                scores.insert(
                    team_name.clone(),
                    scores.get(&team_name).unwrap() + innings.score.runs,
                );
            } else {
                scores.insert(team_name, innings.score.runs);
            };
        }
        if scores.len() < 2 {
            self.outcome = Some(Outcome {
                draw: true,
                tie: false,
                winner: None,
                runs_margin: None,
                wickets_margin: None,
            });
            return;
        }
        let team_a = teams[0].clone();
        let team_b = teams[1].clone();

        self.outcome = match scores
            .get(&team_a)
            .unwrap()
            .cmp(scores.get(&team_b).unwrap())
        {
            std::cmp::Ordering::Greater => {
                Some(self.get_margin(team_a, team_b, batting_team, scores))
            }
            std::cmp::Ordering::Equal => Some(Outcome {
                ..Default::default()
            }),
            std::cmp::Ordering::Less => Some(self.get_margin(team_b, team_a, batting_team, scores)),
        };
    }
}

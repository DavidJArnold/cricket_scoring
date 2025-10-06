#![allow(dead_code)]

// Module used to parse cricsheet files into native types

use crate::scoring::{
    ball::{BallEvents, BallOutcome, Wicket as LibWicket},
    innings::Innings,
    player::{Player, Team},
    r#match::{Match, MatchResult, MatchType, WinMargin},
};
use chrono::NaiveDate;
use serde::Deserialize;
use std::{collections::HashMap, fmt};

mod custom_deserialisers;
use custom_deserialisers::{deserialize_to_option_string, deserialize_to_string};

pub mod utils;

#[derive(Deserialize, Debug)]
pub struct Cricsheet {
    pub meta: CricsheetMeta,
    pub info: CricsheetInfo,
    pub innings: Vec<CricsheetInnings>,
}

impl Cricsheet {
    pub fn create_game(&self) -> Match {
        let team1 = self.info.clone().team(&self.info.teams[0]);
        let team2 = self.info.clone().team(&self.info.teams[1]);

        let match_type = match self.info.match_type.to_lowercase().as_str() {
            "test" => MatchType::Test,
            "odi" => MatchType::OD,
            "t20" => MatchType::T20,
            _ => MatchType::Other(self.info.match_type.clone()),
        };

        let mut cricket_match = Match::new(
            String::from("1"),
            format!("{} vs {}", self.info.teams[0], self.info.teams[1]),
            match_type,
            team1,
            team2,
        );

        if let Some(venue) = &self.info.venue {
            cricket_match = cricket_match.with_venue(venue.clone());
        }

        if let Some(first_date) = self.info.dates.first() {
            cricket_match = cricket_match.with_date(first_date.to_string());
        }

        if let Some(event) = &self.info.event {
            cricket_match.with_event(event.name.clone());
        }

        cricket_match
    }
}

#[derive(Deserialize, Debug)]
pub struct CricsheetMeta {
    pub data_version: String,
    pub created: String,
    pub revision: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CricsheetInfo {
    pub balls_per_over: i32,
    pub bowl_out: Option<Vec<BowlOut>>,
    pub city: Option<String>,
    pub dates: Vec<NaiveDate>,
    pub event: Option<Event>,
    pub gender: String,
    pub match_type: String,
    pub match_type_number: Option<i32>,
    pub missing: Option<Vec<Missing>>,
    pub officials: Option<Officials>,
    pub outcome: Outcome,
    pub overs: Option<i32>,
    pub player_of_match: Option<Vec<String>>,
    pub players: HashMap<String, Vec<String>>,
    pub registry: Registry,
    #[serde(deserialize_with = "deserialize_to_string")]
    pub season: String,
    pub supersubs: Option<HashMap<String, String>>,
    pub team_type: String,
    pub teams: Vec<String>,
    pub toss: Toss,
    pub venue: Option<String>,
}

impl CricsheetInfo {
    pub fn team(self, name: &String) -> Team {
        Team {
            name: name.clone(),
            players: self
                .players
                .get(name)
                .unwrap()
                .iter()
                .map(|x| Player::new(x.clone()))
                .collect::<Vec<Player>>(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CricsheetInnings {
    pub team: String,
    pub overs: Option<Vec<Over>>,
    pub absent_hurt: Option<Vec<String>>,
    pub penalty_runs: Option<PenaltyRuns>,
    pub declared: Option<bool>,
    pub forfeited: Option<bool>,
    pub powerplays: Option<Vec<Powerplay>>,
    pub miscounted_overs: Option<HashMap<String, MiscountedOver>>,
    pub target: Option<Target>,
    pub super_over: Option<bool>,
}

impl CricsheetInnings {
    pub fn process_innings(&self, cricket_match: &mut Match) {
        // initialise the Innings object
        let batting_team_name = &self.team;
        let batting_team = if batting_team_name == &cricket_match.team1.name {
            cricket_match.team1.clone()
        } else {
            cricket_match.team2.clone()
        };
        let bowling_team = if batting_team_name == &cricket_match.team1.name {
            cricket_match.team2.clone()
        } else {
            cricket_match.team1.clone()
        };

        let mut innings = Innings::new(batting_team.clone(), bowling_team.clone());

        // check for penalty runs
        if self.penalty_runs.is_some() {
            innings.score.runs = self.penalty_runs.as_ref().unwrap().pre.unwrap_or_default();
        }

        // iterate through overs and balls
        for over in self.overs.clone().unwrap_or_default() {
            for ball in &over.deliveries {
                // Look up the actual striker and non-striker from the delivery data
                let striker = batting_team
                    .players
                    .iter()
                    .find(|p| p.name == ball.batter)
                    .expect("Batter from delivery not found in batting team")
                    .clone();

                let non_striker = batting_team
                    .players
                    .iter()
                    .find(|p| p.name == ball.non_striker)
                    .expect("Non-striker from delivery not found in batting team")
                    .clone();

                let bowler = bowling_team
                    .players
                    .iter()
                    .find(|p| p.name == ball.bowler)
                    .unwrap()
                    .clone();

                let ball_outcome = ball.parse(striker, non_striker, bowler);
                innings.score_ball(&ball_outcome);
            }
            innings.over();
        }
        innings.finished = true;

        // check for penalty runs
        if self.penalty_runs.is_some() {
            innings.score.runs += self.penalty_runs.as_ref().unwrap().post.unwrap_or_default();
        }
        cricket_match.add_innings(innings.clone());
    }

    pub fn process_innings_with_states(&self, team1: Team, team2: Team) -> Vec<Innings> {
        let mut states = Vec::new();

        // initialise the Innings object
        let batting_team_name = &self.team;
        let batting_team = if batting_team_name == &team1.name {
            team1.clone()
        } else {
            team2.clone()
        };
        let bowling_team = if batting_team_name == &team1.name {
            team2.clone()
        } else {
            team1.clone()
        };

        let mut innings = Innings::new(batting_team.clone(), bowling_team.clone());

        // check for penalty runs
        if self.penalty_runs.is_some() {
            innings.score.runs = self.penalty_runs.as_ref().unwrap().pre.unwrap_or_default();
        }

        // iterate through overs and balls
        for over in self.overs.clone().unwrap_or_default() {
            for ball in &over.deliveries {
                // Look up the actual striker and non-striker from the delivery data
                let striker = batting_team
                    .players
                    .iter()
                    .find(|p| p.name == ball.batter)
                    .expect("Batter from delivery not found in batting team")
                    .clone();

                let non_striker = batting_team
                    .players
                    .iter()
                    .find(|p| p.name == ball.non_striker)
                    .expect("Non-striker from delivery not found in batting team")
                    .clone();

                let bowler = bowling_team
                    .players
                    .iter()
                    .find(|p| p.name == ball.bowler)
                    .unwrap()
                    .clone();

                let ball_outcome = ball.parse(striker, non_striker, bowler);
                innings.score_ball(&ball_outcome);
                states.push(innings.clone());
            }
            innings.over();
        }
        innings.finished = true;

        // check for penalty runs
        if self.penalty_runs.is_some() {
            innings.score.runs += self.penalty_runs.as_ref().unwrap().post.unwrap_or_default();
        }

        // Update the last state with the final innings (with finished flag and post-penalty runs)
        if let Some(last) = states.last_mut() {
            *last = innings;
        }

        states
    }
}

#[derive(Deserialize, Debug)]
pub struct PenaltyRuns {
    pub pre: Option<i32>,
    pub post: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Powerplay {
    pub from: f32,
    pub to: f32,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Deserialize, Debug)]
pub struct MiscountedOver {
    #[serde(deserialize_with = "deserialize_to_string")]
    pub balls: String,
    pub umpire: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Target {
    pub overs: Option<f32>,
    pub runs: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Over {
    pub over: i32,
    pub deliveries: Vec<Delivery>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Delivery {
    pub batter: String,
    pub bowler: String,
    pub extras: Option<Extras>,
    pub non_striker: String,
    pub replacements: Option<Replacement>,
    pub review: Option<Review>,
    pub runs: Runs,
    pub wickets: Option<Vec<Wicket>>,
}

impl Delivery {
    pub fn parse(&self, striker: Player, non_striker: Player, bowler: Player) -> BallOutcome {
        let mut ball_events: Vec<BallEvents> = Vec::new();
        if self.extras.is_some() {
            if self.extras.clone().unwrap().byes.is_some() {
                ball_events.push(BallEvents::Bye(self.extras.clone().unwrap().byes.unwrap()));
            }
            if self.extras.clone().unwrap().legbyes.is_some() {
                ball_events.push(BallEvents::LegBye(
                    self.extras.clone().unwrap().legbyes.unwrap(),
                ));
            }
            if self.extras.clone().unwrap().wides.is_some() {
                ball_events.push(BallEvents::Wide(
                    self.extras.clone().unwrap().wides.unwrap(),
                ));
            }
            if self.extras.clone().unwrap().penalty.is_some() {
                ball_events.push(BallEvents::Penalty(
                    self.extras.clone().unwrap().penalty.unwrap(),
                ));
            }
            if self.extras.clone().unwrap().noballs.is_some() {
                ball_events.push(BallEvents::NoBall(
                    self.extras.clone().unwrap().noballs.unwrap(),
                ));
            }
        }
        if self.wickets.is_some() {
            ball_events.push(BallEvents::Wicket(
                self.wickets
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|x| LibWicket {
                        player_out: x.player_out,
                        kind: x.kind,
                    })
                    .collect(),
            ));
        }
        if self.runs.batter == 4 && !self.runs.non_boundary.unwrap_or(false) {
            ball_events.push(BallEvents::Four);
        }
        if self.runs.batter == 6 && !self.runs.non_boundary.unwrap_or(false) {
            ball_events.push(BallEvents::Six);
        }

        let ball_outcome =
            BallOutcome::new(self.runs.batter, ball_events, striker, non_striker, bowler);
        ball_outcome.validate().unwrap();
        ball_outcome
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Extras {
    pub byes: Option<i32>,
    pub legbyes: Option<i32>,
    pub noballs: Option<i32>,
    pub penalty: Option<i32>,
    pub wides: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Replacement {
    pub role: Option<Vec<ReplacementRole>>,
    #[serde(rename = "match")]
    pub game: Option<Vec<ReplacementMatch>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReplacementRole {
    #[serde(rename = "in")]
    pub player_in: String,
    pub out: Option<String>,
    pub reason: String,
    pub role: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReplacementMatch {
    #[serde(rename = "in")]
    pub player_in: String,
    pub out: Option<String>,
    pub reason: String,
    pub team: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Review {
    pub batter: String,
    pub by: String,
    pub decision: String,
    pub umpire: Option<String>,
    pub umpires_call: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Runs {
    pub batter: i32,
    pub extras: i32,
    pub non_boundary: Option<bool>,
    pub total: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Wicket {
    pub fielders: Option<Vec<Fielder>>,
    pub kind: String,
    pub player_out: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Fielder {
    pub name: Option<String>,
    pub substitute: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BowlOut {
    pub bowler: String,
    pub outcome: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Event {
    pub name: String,
    pub match_number: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_to_option_string")]
    pub group: Option<String>,
    pub stage: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Missing {
    StringField(String),
    Powerplays(MissingPowerplays),
}

#[derive(Deserialize, Debug, Clone)]
pub struct MissingPowerplays {
    powerplays: HashMap<String, Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Officials {
    pub match_referees: Option<Vec<String>>,
    pub reserve_umpires: Option<Vec<String>>,
    pub tv_umpires: Option<Vec<String>>,
    pub umpires: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Method {
    pub innings: Option<i32>,
    pub runs: Option<i32>,
    pub wickets: Option<i32>,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.runs.is_some() {
            if self.innings.is_some() {
                return write!(f, "Won by an innings and {} runs", self.runs.unwrap());
            };
            return write!(f, "Won by {} runs", self.runs.unwrap());
        }
        if self.wickets.is_some() {
            if self.innings.is_some() {
                return write!(f, "Won by an innings and {} wickets", self.wickets.unwrap());
            };
            return write!(f, "Won by {} wickets", self.wickets.unwrap());
        }
        panic!("No winning information");
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Outcome {
    pub by: Option<Method>,
    pub bowl_out: Option<String>,
    pub eliminator: Option<String>,
    pub method: Option<String>,
    pub result: Option<String>,
    pub winner: Option<String>,
}

impl Outcome {
    pub fn create_match_result(&self, team1_name: &str, team2_name: &str) -> MatchResult {
        // Handle special cases first
        if self.result == Some(String::from("draw")) {
            return MatchResult::Draw;
        }

        if self.result == Some(String::from("tie")) {
            let method = self.method.as_ref().map(|m| m.clone());
            return MatchResult::Tie { method };
        }

        if self.result == Some(String::from("no result")) {
            return MatchResult::NoResult;
        }

        // Handle wins with margins
        if let Some(winner) = &self.winner {
            let margin = if let Some(by) = self.by {
                if let Some(runs) = by.runs {
                    WinMargin::Runs(runs as u32)
                } else if let Some(wickets) = by.wickets {
                    WinMargin::Wickets(wickets as u8)
                } else {
                    // No margin specified - likely an awarded match
                    WinMargin::Award
                }
            } else {
                // No "by" field - match was awarded
                WinMargin::Award
            };

            let method = self.method.as_ref().map(|m| m.clone());
            if winner == team1_name {
                MatchResult::Team1Won { margin, method }
            } else if winner == team2_name {
                MatchResult::Team2Won { margin, method }
            } else {
                // Winner name doesn't match either team, fall back to no result
                MatchResult::NoResult
            }
        } else {
            MatchResult::NoResult
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Registry {
    pub people: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Toss {
    pub decision: String,
    pub winner: String,
    pub uncontested: Option<bool>,
}

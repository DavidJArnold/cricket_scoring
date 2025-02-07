#![allow(dead_code)]

use chrono::NaiveDate;
use cricket_scoring::scoring::player::{Player, Team};
use serde::Deserialize;
use std::{collections::HashMap, fmt};

mod custom_deserialisers;
use crate::cricsheet_lib::custom_deserialisers::{
    deserialize_to_option_string, deserialize_to_string,
};

#[derive(Deserialize, Debug)]
pub struct Cricsheet {
    pub meta: CricsheetMeta,
    pub info: CricsheetInfo,
    pub innings: Vec<CricsheetInnings>,
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
                .collect::<Vec<Player>>()
                .try_into()
                .unwrap(),
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

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct Outcome {
    pub by: Option<Method>,
    pub bowl_out: Option<String>,
    pub eliminator: Option<String>,
    pub method: Option<String>,
    pub result: Option<String>,
    pub winner: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
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
            } else {
                return write!(f, "Won by {} runs", self.runs.unwrap());
            }
        }
        if self.wickets.is_some() {
            if self.innings.is_some() {
                return write!(f, "Won by an innings and {} wickets", self.wickets.unwrap());
            } else {
                return write!(f, "Won by {} wickets", self.wickets.unwrap());
            }
        }
        panic!("No winning information");
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

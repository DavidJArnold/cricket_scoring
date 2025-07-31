#![allow(dead_code)]

// Module used to parse cricsheet files into native types

use chrono::NaiveDate;
use cricket_scoring::scoring::{
    ball::{BallEvents, BallOutcome, Wicket as LibWicket},
    game::{Game, Meta, Outcome as GameOutcome},
    innings::Innings,
    player::{Player, Team},
};
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

impl Cricsheet {
    pub fn create_game(&self) -> Game {
        let cricsheet_teams = self.info.teams.clone();
        let teams: HashMap<String, Team> = cricsheet_teams
            .iter()
            .map(|x| (x.clone(), self.info.clone().team(x)))
            .collect();

        Game::new(
            Meta {
                venue: self.info.venue.clone(),
            },
            teams,
        )
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
    pub fn process_innings(&self, cricket_match: &mut Game) {
        // initialise the Innings object
        let batting_team_name = &self.team;
        let mut batting_team: Option<Team> = None;
        let mut bowling_team: Option<Team> = None;
        for (team_name, team) in &cricket_match.teams {
            if team_name == batting_team_name {
                batting_team = Some(team.clone());
            } else {
                bowling_team = Some(team.clone());
            }
        }
        let batting_team = batting_team.unwrap();
        let bowling_team = bowling_team.unwrap();

        let mut innings = Innings::new(batting_team.clone(), bowling_team.clone());

        // check for penalty runs
        if self.penalty_runs.is_some() {
            innings.score.runs = self.penalty_runs.as_ref().unwrap().pre.unwrap_or_default();
        }

        // iterate through overs and balls
        for over in self.overs.clone().unwrap_or_default() {
            for ball in &over.deliveries {
                let ball_outcome = ball.parse(
                    batting_team.players.get(innings.on_strike).unwrap().clone(),
                    batting_team
                        .players
                        .get(innings.off_strike)
                        .unwrap()
                        .clone(),
                );
                innings.score_ball(&ball_outcome);
            }
            innings.over();
        }
        innings.finished = true;

        // check for penalty runs
        if self.penalty_runs.is_some() {
            innings.score.runs += self.penalty_runs.as_ref().unwrap().post.unwrap_or_default();
        }
        cricket_match.innings.push(innings.clone());
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
    pub fn parse(&self, striker: Player, non_striker: Player) -> BallOutcome {
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
                    .map(|x| LibWicket { player_out:x.player_out, kind:x.kind})
                    .collect(),
            ));
        }
        if self.runs.batter == 4 && !self.runs.non_boundary.unwrap_or(false) {
            ball_events.push(BallEvents::Four);
        }
        if self.runs.batter == 6 && !self.runs.non_boundary.unwrap_or(false) {
            ball_events.push(BallEvents::Six);
        }

        let ball_outcome = BallOutcome::new(self.runs.batter, ball_events, striker, non_striker);
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
    pub fn create_outcome(&self) -> GameOutcome {
        match self.by {
            Some(x) => GameOutcome {
                draw: self.result == Some(String::from("draw")),
                tie: self.result == Some(String::from("tie")),
                winner: self.winner.clone(),
                method: self.method.clone(),
                runs_margin: x.runs,
                wickets_margin: x.wickets,
                innings_win: x.innings.is_some(),
                result: self.result != Some(String::from("no result")),
            },
            None => GameOutcome {
                draw: self.result == Some(String::from("draw")),
                tie: self.result == Some(String::from("tie")),
                winner: self.winner.clone(),
                method: self.method.clone(),
                ..Default::default()
            },
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

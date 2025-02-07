mod cricsheet_lib;
use cricket_scoring::scoring::game::GameMeta;
use cricket_scoring::scoring::player::Team;
use cricsheet_lib::{Cricsheet, Delivery};
use std::collections::HashMap;
use std::{fs::File, io::Read};

use cricket_scoring::scoring::ball::{BallEvents, BallOutcome};
use cricket_scoring::scoring::{game::Game, innings::Innings};

fn parse_ball(ball: &Delivery) -> BallOutcome {
    let mut ball_events: Vec<BallEvents> = Vec::new();
    if ball.extras.is_some() {
        if ball.extras.clone().unwrap().byes.is_some() {
            ball_events.push(BallEvents::Bye);
        }
        if ball.extras.clone().unwrap().legbyes.is_some() {
            ball_events.push(BallEvents::LegBye);
        }
        if ball.extras.clone().unwrap().wides.is_some() {
            ball_events.push(BallEvents::Wide);
        }
        if ball.extras.clone().unwrap().noballs.is_some() {
            ball_events.push(BallEvents::NoBall);
        }
    }
    if ball.wickets.is_some() {
        ball_events.push(BallEvents::Wicket);
    }
    if ball.runs.batter == 4 && !ball.runs.non_boundary.unwrap_or(false) {
        ball_events.push(BallEvents::Four);
    }
    if ball.runs.batter == 6 && !ball.runs.non_boundary.unwrap_or(false) {
        ball_events.push(BallEvents::Six);
    }

    let ball_outcome = BallOutcome::new(ball.runs.batter, ball_events);
    ball_outcome.validate().unwrap();
    ball_outcome
}

fn main() {
    // parse a set of cricsheet games
    let filename = "examples/all_matches";
    let num_files = std::fs::read_dir(filename).unwrap().count();
    let mut read_files = 0;
    for file in std::fs::read_dir(filename).unwrap() {
        let x = file.unwrap();
        if x.path().to_str().unwrap().ends_with("txt") {
            continue;
        }
        let mut data = String::new();
        let mut file = File::open(x.path()).unwrap();
        let _ = &file.read_to_string(&mut data);

        // parse the game into a Cricsheet object
        let json: serde_json::Value = serde_json::from_str(&data).unwrap();
        let cricsheet: Cricsheet = serde_json::from_value(json).unwrap();

        // Now construct a Game object for this game
        //
        // First get the teams
        let cricsheet_teams = cricsheet.info.teams.clone();
        let teams: HashMap<String, Team> = HashMap::from_iter(
            cricsheet_teams
                .iter()
                .map(|x| (x.clone(), cricsheet.info.clone().team(&x))),
        );

        // then initialise the game
        let mut cricket_match = Game {
            innings: vec![],
            meta: GameMeta {
                venue: cricsheet.info.venue,
            },
            outcome: None,
            teams,
        };

        // Now go through the innings'
        for innings_data in &cricsheet.innings {
            // initialise the Innings object
            let batting_team_name = innings_data.team.clone();
            let bowling_team_name = cricsheet
                .info
                .teams
                .iter()
                .filter(|x| **x != batting_team_name)
                .next()
                .unwrap()
                .clone();
            let mut innings = Innings::new(
                cricket_match.teams.get(&batting_team_name).unwrap().clone(),
                cricket_match.teams.get(&bowling_team_name).unwrap().clone(),
            );

            // iterate through overs and balls
            for over in innings_data.overs.clone().unwrap_or(vec![]) {
                for ball in &over.deliveries {
                    let ball_outcome = parse_ball(ball);
                    innings.score_ball(&ball_outcome);
                }
                innings.over();
            }
            innings.finished = true;
            // println!("{}\n{}", innings_data.team, innings);
            cricket_match.innings.push(innings);
        }

        cricket_match.score();

        let by = match cricsheet.info.outcome.by {
            Some(x) => format!("{}", &x),
            None => format!("No winner"),
        };
        println!(
            "CRICSHEET: {} {by} {}",
            cricsheet
                .info
                .outcome
                .winner
                .unwrap_or("NO WINNER".to_string()),
            cricsheet.info.outcome.method.unwrap_or(String::new())
        );

        let res = cricket_match.outcome.unwrap();
        println!(
            "MY SCORES: {} {:?} wickets or {:?} runs",
            res.winner.unwrap_or("NO WINNER".to_string()),
            res.wickets_margin.unwrap_or(0),
            res.runs_margin.unwrap_or(0),
        );
        read_files += 1;
        if read_files % 20 == 0 {
            println!("{read_files}/{num_files}");
            break;
        }
    }
}

mod cricsheet_lib;
use cricsheet_lib::Cricsheet;
use std::{fs::File, io::Read};

use cricket_scoring::scoring::{innings::Innings, player::Player, BallEvents, BallOutcome};

fn main() {
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

        let json: serde_json::Value = serde_json::from_str(&data).unwrap();
        let cricsheet: Cricsheet = serde_json::from_value(json).unwrap();

        for innings_data in &cricsheet.innings {

            let batting_team_name = innings_data.team.clone();
            let bowling_team_name = &cricsheet.info.teams.iter().filter(|x| **x != batting_team_name).next().unwrap();

            let batters = cricsheet.info.players.get(&batting_team_name).unwrap().iter();
            let mut batting_team = vec![];
            for player in batters {
                batting_team.push(Player::new(player.to_string()));
            }

            let bowlers = cricsheet.info.players.get(*bowling_team_name).unwrap().iter();
            let mut bowling_team = vec![];
            for player in bowlers {
                bowling_team.push(Player::new(player.to_string()));
            }
            if batting_team.len() != 11 || bowling_team.len() != 11 {
                continue
            }

            let mut innings = Innings::new(batting_team.try_into().unwrap(), bowling_team.try_into().unwrap());
            for over in innings_data.overs.clone().unwrap_or(vec![]) {
                for ball in &over.deliveries {
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
                    innings.score_ball(&ball_outcome);

                    if innings.score.wickets_lost == 10 {
                        break;
                    }
                }
                innings.over();
            }
            println!("{}", innings);
        }
        read_files += 1;
        if read_files % 5 == 0 {
            println!("{read_files}/{num_files}");
        }
    }
}

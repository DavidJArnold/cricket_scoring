use std::fs::{DirEntry, File};
use std::io::Read;

use crate::cricsheet_lib::{Cricsheet, CricsheetInnings};
use cricket_scoring::scoring::innings::Innings;

pub fn compare_results(innings_data: &CricsheetInnings, innings: &Innings) {
    // print  cricsheet results
    let mut cricsheet_runs: i32 = innings_data
        .overs
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|x| x.deliveries.iter().map(|z| z.runs.total).sum::<i32>())
        .sum::<i32>();
    if innings_data.penalty_runs.is_some() {
        cricsheet_runs += innings_data
            .penalty_runs
            .as_ref()
            .unwrap()
            .pre
            .unwrap_or_default()
            + innings_data
                .penalty_runs
                .as_ref()
                .unwrap()
                .post
                .unwrap_or_default();
    }
    let cricsheet_wickets: usize = innings_data
        .overs
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|x| {
            x.deliveries
                .iter()
                .map(|z| z.wickets.clone().unwrap_or_default().len())
                .sum::<usize>()
        })
        .sum();
    if cricsheet_runs != innings.score.runs {
        println!(
            "{} -> CRICSHEET: {}/{} ME: {}/{}",
            innings.batting_team.name,
            cricsheet_wickets,
            cricsheet_runs,
            innings.score.wickets_lost,
            innings.score.runs
        );
        println!(
            "({} runs {} extras) ({} runs {} extras)",
            innings_data
                .overs
                .clone()
                .unwrap()
                .iter()
                .map(|x| x.deliveries.iter().map(|z| z.runs.total).sum::<i32>())
                .sum::<i32>(),
            innings_data
                .overs
                .clone()
                .unwrap()
                .iter()
                .map(|x| x.deliveries.iter().map(|z| z.runs.extras).sum::<i32>())
                .sum::<i32>(),
            innings.score.runs,
            innings.score.wides
                + innings.score.no_balls
                + innings.score.byes
                + innings.score.leg_byes,
        );
        println!("{:?}", innings_data.penalty_runs);
        // println!("{innings}");
    }
}

pub fn get_cricsheet_game(directory_entry: &DirEntry) -> Option<Cricsheet> {
    if directory_entry.path().to_str().unwrap().ends_with("txt") {
        return None;
    }

    let mut data = String::new();
    let mut file = File::open(directory_entry.path()).unwrap();
    let _ = &file.read_to_string(&mut data);

    // parse the game into a Cricsheet object
    let json: serde_json::Value = serde_json::from_str(&data).unwrap();
    Some(serde_json::from_value(json).unwrap())
}

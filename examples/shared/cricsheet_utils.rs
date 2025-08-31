use std::fs::{DirEntry, File};
use std::io::Read;

use crate::cricsheet_lib::{Cricsheet, CricsheetInnings, Event};
use cricket_scoring::scoring::game::Game;
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

pub fn format_cricsheet_result(cricsheet: &Cricsheet) -> String {
    let by = match cricsheet.info.outcome.by {
        Some(x) => format!("{}", &x),
        None => cricsheet.info.outcome.result.clone().unwrap_or_default(),
    };
    format!(
        "{} {by} {}",
        cricsheet
            .info
            .outcome
            .winner
            .clone()
            .unwrap_or("NO WINNER".to_string()),
        cricsheet.info.outcome.method.clone().unwrap_or_default()
    )
}

pub fn format_result(cricket_match: &Game) -> String {
    let res = cricket_match.outcome.as_ref().unwrap();
    let mut innings_win_text = String::new();
    if res.innings_win {
        innings_win_text = String::from("an innings and ");
    }
    let draw_win;
    let mut margin = String::new();
    if res.draw {
        draw_win = "Draw";
    } else if res.tie {
        draw_win = "tie";
    } else if !res.result {
        draw_win = "no result";
    } else if res.method.clone().unwrap_or_default() == "Awarded" {
        draw_win = "";
    } else if res.method.clone().unwrap_or_default() == "Lost fewer wickets" {
        draw_win = "";
    } else {
        draw_win = "Won by";
    };
    let mut run_wickets = String::new();
    if res.runs_margin.is_some() {
        run_wickets = String::from("runs");
        margin = format!("{}", res.runs_margin.unwrap());
    };
    if res.wickets_margin.is_some() {
        run_wickets = String::from("wickets");
        margin = format!("{}", res.wickets_margin.unwrap());
    };
    format!(
        "{} {draw_win} {}{} {} {}",
        res.winner.clone().unwrap_or("NO WINNER".to_string()),
        innings_win_text,
        margin,
        run_wickets,
        res.method.clone().unwrap_or_default(),
    )
}

pub fn print_diffs(
    cricsheet: &Cricsheet,
    cricket_match: &Game,
    cricsheet_result: &str,
    my_result: &str,
    file: DirEntry,
) {
    println!("---------------------------------");
    println!(
        "{:?} {}",
        cricsheet.info.dates,
        cricsheet
            .info
            .event
            .clone()
            .unwrap_or_else(|| Event {
                name: String::new(),
                ..Default::default()
            })
            .name
    );
    println!(
        "MATCH {}\nCRICSHEET: {}\nMY SCORE: {}",
        file.file_name().into_string().unwrap(),
        cricsheet_result,
        my_result
    );
    println!(
        "{}",
        cricket_match
            .innings
            .iter()
            .map(|x| x.score.summary())
            .fold(String::new(), |x, y| vec![x, "\n".to_string(), y]
                .into_iter()
                .collect::<String>())
    );
}

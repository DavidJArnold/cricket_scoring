use std::fs::{DirEntry, File};
use std::io::Read;

use crate::cricsheet_lib::{Cricsheet, CricsheetInnings, Event};
use cricket_scoring::scoring::innings::Innings;
use cricket_scoring::scoring::r#match::{Match, MatchResult, WinMargin};

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
            .unwrap_or(String::new()),
        cricsheet.info.outcome.method.clone().unwrap_or_default()
    )
}

pub fn format_result(cricket_match: &Match) -> String {
    let Some(result) = &cricket_match.result else {
        return "NO RESULT".to_string();
    };

    let is_innings_win = cricket_match.is_innings_victory();

    match result {
        MatchResult::Team1Won { margin, method } => {
            let margin_text = if is_innings_win {
                match margin {
                    WinMargin::Runs(runs) => format!("by an innings and {} runs", runs),
                    WinMargin::Wickets(_) => {
                        // For innings victories, wickets margin doesn't make sense
                        // Calculate the correct run margin instead
                        let run_margin = cricket_match.team1_total_runs() - cricket_match.team2_total_runs();
                        eprintln!("Warning: Innings victory with wickets margin detected - correcting to {} run margin", run_margin);
                        format!("by an innings and {} runs", run_margin)
                    },
                    WinMargin::Award => String::new(),
                }
            } else {
                match margin {
                    WinMargin::Runs(runs) => format!("by {} runs", runs),
                    WinMargin::Wickets(wickets) => format!("by {} wickets", wickets),
                    WinMargin::Award => String::new(),
                }
            };
            let method_text = match method {
                Some(m) => format!(" {}", m),
                None => String::new(),
            };
            match margin {
                WinMargin::Award => format!("{} {}", cricket_match.team1.name, method_text),
                _ => format!("{} Won {}{}", cricket_match.team1.name, margin_text, method_text)
                }
        }
        MatchResult::Team2Won { margin, method } => {
            let margin_text = if is_innings_win {
                match margin {
                    WinMargin::Runs(runs) => format!("by an innings and {} runs", runs),
                    WinMargin::Wickets(_) => {
                        // For innings victories, wickets margin doesn't make sense
                        // Calculate the correct run margin instead
                        let run_margin = cricket_match.team2_total_runs() - cricket_match.team1_total_runs();
                        eprintln!("Warning: Innings victory with wickets margin detected - correcting to {} run margin", run_margin);
                        format!("by an innings and {} runs", run_margin)
                    },
                    WinMargin::Award => String::new(),
                }
            } else {
                match margin {
                    WinMargin::Runs(runs) => format!("by {} runs", runs),
                    WinMargin::Wickets(wickets) => format!("by {} wickets", wickets),
                    WinMargin::Award => String::new(),
                }
            };
            let method_text = match method {
                Some(m) => format!(" {}", m),
                None => String::new(),
            };
            match margin {
                WinMargin::Award => format!("{} {}", cricket_match.team2.name, method_text),
                _ => format!("{} Won {}{}", cricket_match.team2.name, margin_text, method_text)
                }
        }
        MatchResult::Tie { method } => {
            match method {
                Some(m) => format!("Tie {}", m),
                None => String::from("Tie"),
            }
        }
        MatchResult::Draw => "Draw".to_string(),
        MatchResult::NoResult => "no result".to_string(),
    }
}

pub fn print_diffs(
    cricsheet: &Cricsheet,
    cricket_match: &Match,
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

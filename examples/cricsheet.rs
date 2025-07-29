mod cricsheet_lib;
mod cricsheet_utils;
use cricsheet_utils::{compare_results, get_cricsheet_game};

fn main() {
    // parse a set of cricsheet games
    let cricsheet_directory = "examples/all_matches";
    let mut read_files = 0;
    let mut correct_result = 0;
    for file in std::fs::read_dir(cricsheet_directory).unwrap() {
        let cricsheet = get_cricsheet_game(file.as_ref().expect("")).unwrap();

        // Now construct a Game object for this game
        let mut cricket_match = cricsheet.create_game();

        // Now go through the innings'
        for innings_data in &cricsheet.innings {
            innings_data.process_innings(&mut cricket_match);
            let innings = cricket_match.innings.last().unwrap();

            compare_results(innings_data, innings);
        }

        cricket_match.score(cricsheet.info.outcome.create_outcome());

        let by = match cricsheet.info.outcome.by {
            Some(x) => format!("{}", &x),
            None => cricsheet.info.outcome.result.unwrap_or_default(),
        };
        let cricsheet_result = format!(
            "{} {by} {}",
            cricsheet
                .info
                .outcome
                .winner
                .unwrap_or("NO WINNER".to_string()),
            cricsheet.info.outcome.method.unwrap_or_default()
        );

        let res = cricket_match.outcome.unwrap();
        let mut innings_win_text = String::new();
        if res.innings_win {
            innings_win_text = String::from("an innings and ");
        }
        let draw_win;
        if res.draw {
            draw_win = "Draw";
        } else if res.tie {
            draw_win = "Tie";
        } else {
            draw_win = "Won by";
        };
        let mut run_wickets = String::new();
        if res.runs_margin.is_some() {
            run_wickets = String::from("runs");
        } else if res.wickets_margin.is_some() {
            run_wickets = String::from("wickets");
        }
        let my_result = format!(
            "{} {draw_win} {}{} {} {}",
            res.winner.unwrap_or("NO WINNER".to_string()),
            innings_win_text,
            res.wickets_margin
                .unwrap_or(res.runs_margin.unwrap_or_default()),
            run_wickets,
            res.method.unwrap_or_default(),
        );
        if my_result
            .trim_end()
            .strip_suffix('0')
            .unwrap_or(&my_result)
            .trim_end()
            .to_lowercase()
            == cricsheet_result.trim_end().to_lowercase()
        {
            correct_result += 1;
        } else {
            println!(
                "MATCH {}\nCRICSHEET: {}\nMY SCORE: {}",
                file.unwrap().file_name().into_string().unwrap(),
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

        read_files += 1;
    }
    println!(
        "Got {correct_result} right out of {read_files} games ({:.2}%)",
        f64::from(correct_result) / f64::from(read_files) * 100.0
    );
}

mod cricsheet_lib;
#[path = "shared/cricsheet_utils.rs"]
mod cricsheet_utils;
use cricsheet_utils::{
    compare_results, format_cricsheet_result, format_result, get_cricsheet_game, print_diffs,
};

fn main() {
    // parse a set of cricsheet games
    let cricsheet_directory = "examples/all_matches";
    let mut read_files = 0;
    let mut correct_result = 0;
    for file in std::fs::read_dir(cricsheet_directory).unwrap() {
        let cricsheet_record = get_cricsheet_game(file.as_ref().expect(""));
        if cricsheet_record.is_none() {
            continue;
        };
        let cricsheet = cricsheet_record.unwrap();

        // Now construct a Game object for this game
        let mut cricket_match = cricsheet.create_game();

        // Now go through the innings'
        for innings_data in &cricsheet.innings {
            innings_data.process_innings(&mut cricket_match);
            let innings = cricket_match.innings.last().unwrap();

            compare_results(innings_data, innings);
        }

        // Convert cricsheet outcome to MatchResult and set it
        let match_result = cricsheet
            .info
            .outcome
            .create_match_result(&cricket_match.team1.name, &cricket_match.team2.name);
        cricket_match.set_result(match_result);

        let cricsheet_result = format_cricsheet_result(&cricsheet);

        let my_result = format_result(&cricket_match);

        if my_result
            .trim()
            .strip_suffix('0')
            .unwrap_or(&my_result)
            .replace(" ", "")
            .to_lowercase()
            == cricsheet_result.trim_end().to_lowercase().replace(" ", "")
        {
            correct_result += 1;
        } else {
            print_diffs(
                &cricsheet,
                &cricket_match,
                &cricsheet_result,
                &my_result,
                file.expect(""),
            );
        }

        read_files += 1;
    }
    println!(
        "Got {correct_result} right out of {read_files} games ({:.2}%)",
        f64::from(correct_result) / f64::from(read_files) * 100.0
    );
}

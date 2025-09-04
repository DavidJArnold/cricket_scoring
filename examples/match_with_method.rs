// Example demonstrating the method field in MatchResult
use cricket_scoring::{Match, MatchType, Team, Player, MatchResult, WinMargin};

fn main() {
    // Create teams
    let team1 = Team {
        name: "Australia".to_string(),
        players: vec![
            Player::new("Steve Smith".to_string()),
            Player::new("David Warner".to_string()),
        ],
    };
    
    let team2 = Team {
        name: "England".to_string(),
        players: vec![
            Player::new("Joe Root".to_string()),
            Player::new("Ben Stokes".to_string()),
        ],
    };

    // Create match
    let mut cricket_match = Match::new(
        "AUS_ENG_D_L".to_string(),
        "Australia vs England - Rain Affected".to_string(),
        MatchType::OD,
        team1,
        team2,
    )
    .with_venue("The Oval".to_string())
    .with_date("2025-08-15".to_string());

    // Set result with Duckworth-Lewis method
    let result = MatchResult::Team1Won {
        margin: WinMargin::Runs(28),
        method: Some("D/L".to_string()),
    };
    cricket_match.set_result(result);

    // Display the result
    match &cricket_match.result {
        Some(MatchResult::Team1Won { margin, method }) => {
            let method_text = match method {
                Some(m) => format!(" {}", m),
                None => String::new(),
            };
            match margin {
                WinMargin::Runs(runs) => {
                    println!("Australia won by {} runs{}", runs, method_text);
                }
                WinMargin::Wickets(wickets) => {
                    println!("Australia won by {} wickets{}", wickets, method_text);
                }
                WinMargin::Award => {
                    println!("Australia won by award{}", method_text);
                }
            }
        }
        Some(MatchResult::Team2Won { margin, method }) => {
            let method_text = match method {
                Some(m) => format!(" {}", m),
                None => String::new(),
            };
            match margin {
                WinMargin::Runs(runs) => {
                    println!("England won by {} runs{}", runs, method_text);
                }
                WinMargin::Wickets(wickets) => {
                    println!("England won by {} wickets{}", wickets, method_text);
                }
                WinMargin::Award => {
                    println!("England won by award{}", method_text);
                }
            }
        }
        Some(MatchResult::Tie { method }) => {
            let method_text = match method {
                Some(m) => format!(" {}", m),
                None => String::new(),
            };
            println!("Match tied{}", method_text);
        }
        Some(MatchResult::Draw) => println!("Match drawn"),
        Some(MatchResult::NoResult) => println!("No result"),
        None => println!("Match not completed"),
    }

    // Show serialization with method
    let json = serde_json::to_string_pretty(&cricket_match).unwrap();
    println!("\nSerialized match:");
    println!("{}", json);
}
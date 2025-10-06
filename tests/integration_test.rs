// Integration test to verify the examples work with the new unified API
use cricket_scoring::{Match, MatchResult, MatchType, Player, Team, WinMargin};

#[cfg(feature = "cricsheet")]
use std::fs;

#[test]
fn test_match_creation_and_result_setting() {
    let team1 = Team {
        name: "Team A".to_string(),
        players: vec![Player::new("Player1".to_string())],
    };
    let team2 = Team {
        name: "Team B".to_string(),
        players: vec![Player::new("Player2".to_string())],
    };

    let mut cricket_match = Match::new(
        "test_match".to_string(),
        "Test Match".to_string(),
        MatchType::T20,
        team1,
        team2,
    );

    // Test setting a match result
    let result = MatchResult::Team1Won {
        margin: WinMargin::Runs(25),
        method: None,
    };
    cricket_match.set_result(result);

    assert!(cricket_match.is_completed());

    // Test serialization works
    let json = serde_json::to_string(&cricket_match).unwrap();
    let deserialized: Match = serde_json::from_str(&json).unwrap();

    assert_eq!(cricket_match.id, deserialized.id);
    assert_eq!(cricket_match.title, deserialized.title);
    match deserialized.result.unwrap() {
        MatchResult::Team1Won {
            margin: WinMargin::Runs(runs),
            ..
        } => assert_eq!(runs, 25),
        _ => panic!("Unexpected result"),
    }
}

#[test]
fn test_calculate_result_functionality() {
    use cricket_scoring::Innings;

    let team1 = Team {
        name: "Team A".to_string(),
        players: vec![Player::new("Player1".to_string())],
    };
    let team2 = Team {
        name: "Team B".to_string(),
        players: vec![Player::new("Player2".to_string())],
    };

    let mut cricket_match = Match::new(
        "calc_test".to_string(),
        "Calculation Test".to_string(),
        MatchType::OD,
        team1.clone(),
        team2.clone(),
    );

    // Add innings - Team A scores 200, Team B scores 150 (all out)
    let mut innings1 = Innings::new(team1.clone(), team2.clone());
    innings1.score.runs = 200;
    innings1.score.wickets_left = 5;
    cricket_match.add_innings(innings1);

    let mut innings2 = Innings::new(team2, team1);
    innings2.score.runs = 150;
    innings2.score.wickets_left = 0; // All out
    cricket_match.add_innings(innings2);

    // Use the new calculation method
    cricket_match.calculate_result();

    // Should be Team1Won by 50 runs
    match cricket_match.result.unwrap() {
        MatchResult::Team1Won {
            margin: WinMargin::Runs(runs),
            ..
        } => assert_eq!(runs, 50),
        _ => panic!("Expected Team1 to win by runs"),
    }
}

#[cfg(feature = "cricsheet")]
#[test]
fn test_process_innings_with_states() {
    use cricket_scoring::cricsheet::Cricsheet;
    use serde_json;

    // Load the sample cricsheet JSON file
    let json_content =
        fs::read_to_string("examples/all_matches/1409478.json").expect("Failed to read test file");

    let cricsheet: Cricsheet =
        serde_json::from_str(&json_content).expect("Failed to deserialize cricsheet data");

    // Create teams
    let team1 = cricsheet.info.clone().team(&cricsheet.info.teams[0]);
    let team2 = cricsheet.info.clone().team(&cricsheet.info.teams[1]);

    // Get the first innings
    let first_innings = &cricsheet.innings[0];

    // Count total deliveries in the innings
    let total_deliveries: usize = first_innings
        .overs
        .as_ref()
        .unwrap()
        .iter()
        .map(|over| over.deliveries.len())
        .sum();

    // Call process_innings_with_states
    let states = first_innings.process_innings_with_states(team1.clone(), team2.clone());

    // Test 1: Verify the vector length matches the number of deliveries
    assert_eq!(
        states.len(),
        total_deliveries,
        "Number of states should match number of deliveries"
    );

    // Test 2: Verify runs accumulate correctly
    // Based on the test file: delivery 1 = 0 runs, delivery 2 = 1 run, delivery 3 = 1 run
    assert_eq!(states[0].score.runs, 0, "First delivery should have 0 runs");
    assert_eq!(
        states[1].score.runs, 1,
        "Second delivery should have 1 run total"
    );
    assert_eq!(
        states[2].score.runs, 2,
        "Third delivery should have 2 runs total"
    );

    // Test 3: Verify wickets remain constant (no wickets in this test file)
    for (i, state) in states.iter().enumerate() {
        assert_eq!(
            state.score.wickets_left, 10,
            "State {} should have 10 wickets remaining",
            i
        );
    }

    // Test 4: Verify the final state properties
    let final_state = states.last().unwrap();
    assert!(
        final_state.finished,
        "Final state should be marked as finished"
    );

    // Test 5: Compare with process_innings() result
    let mut cricket_match = Match::new(
        String::from("test"),
        format!("{} vs {}", team1.name, team2.name),
        MatchType::T20,
        team1.clone(),
        team2.clone(),
    );

    first_innings.process_innings(&mut cricket_match);
    let process_innings_result = &cricket_match.innings[0];

    // Final states should match
    assert_eq!(
        final_state.score.runs, process_innings_result.score.runs,
        "Final state runs should match process_innings result"
    );
    assert_eq!(
        final_state.score.wickets_left, process_innings_result.score.wickets_left,
        "Final state wickets should match process_innings result"
    );
    assert_eq!(
        final_state.finished, process_innings_result.finished,
        "Final state finished flag should match process_innings result"
    );
}

#[cfg(feature = "cricsheet")]
#[test]
fn test_cricsheet_batter_scoring_regression() {
    // Regression test to verify the fix for batter scoring bug
    // This test uses actual Cricsheet data to ensure runs are attributed to the correct batters
    use cricket_scoring::cricsheet::Cricsheet;
    use serde_json;

    let json_content =
        fs::read_to_string("examples/all_matches/1409478.json").expect("Failed to read test file");

    let cricsheet: Cricsheet =
        serde_json::from_str(&json_content).expect("Failed to deserialize cricsheet data");

    let mut cricket_match = cricsheet.create_game();

    // Process the innings
    cricsheet.innings[0].process_innings(&mut cricket_match);

    // Verify batter scores match what the Cricsheet data says
    // According to the data:
    // - CJ Bowes faced deliveries: 0 runs, then 1 run = 1 total
    // - HM Nicholls faced delivery: 1 run = 1 total
    let innings = &cricket_match.innings[0];

    let cj_bowes = innings
        .batting_team
        .players
        .iter()
        .find(|p| p.name == "CJ Bowes")
        .expect("CJ Bowes not found");

    let hm_nicholls = innings
        .batting_team
        .players
        .iter()
        .find(|p| p.name == "HM Nicholls")
        .expect("HM Nicholls not found");

    assert_eq!(
        cj_bowes.runs, 1,
        "CJ Bowes should have 1 run (faced 2 balls: 0 runs + 1 run)"
    );
    assert_eq!(
        cj_bowes.balls_faced, 2,
        "CJ Bowes should have faced 2 balls"
    );
    assert_eq!(
        hm_nicholls.runs, 1,
        "HM Nicholls should have 1 run (faced 1 ball: 1 run)"
    );
    assert_eq!(
        hm_nicholls.balls_faced, 1,
        "HM Nicholls should have faced 1 ball"
    );

    // Verify total is correct
    assert_eq!(innings.score.runs, 2, "Total should be 2 runs");
}

// Integration test to verify the examples work with the new unified API
use cricket_scoring::{Match, MatchResult, MatchType, Player, Team, WinMargin};

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

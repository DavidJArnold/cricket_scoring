use cricket_scoring::*;
use serde_json;

#[test]
fn test_player_serialization() {
    let mut player = Player::new("Test Player".to_string());
    player.runs = 50;
    player.balls_faced = 30;
    player.fours = 6;
    player.sixes = 2;
    player.out = true;

    // Test serialization
    let json = serde_json::to_string(&player).unwrap();
    assert!(json.contains("Test Player"));
    assert!(json.contains("50"));

    // Test deserialization
    let deserialized: Player = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "Test Player");
    assert_eq!(deserialized.runs, 50);
    assert_eq!(deserialized.balls_faced, 30);
    assert_eq!(deserialized.fours, 6);
    assert_eq!(deserialized.sixes, 2);
    assert!(deserialized.out);
}

#[test]
fn test_team_serialization() {
    let team = Team {
        name: "Test Team".to_string(),
        players: vec![
            Player::new("Player1".to_string()),
            Player::new("Player2".to_string()),
        ],
    };

    let json = serde_json::to_string(&team).unwrap();
    let deserialized: Team = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.name, "Test Team");
    assert_eq!(deserialized.players.len(), 2);
    assert_eq!(deserialized.players[0].name, "Player1");
    assert_eq!(deserialized.players[1].name, "Player2");
}

#[test]
fn test_current_score_serialization() {
    let mut score = CurrentScore::new();
    score.runs = 150;
    score.wickets_lost = 3;
    score.over = 25;
    score.ball = 4;
    score.wides = 5;
    score.no_balls = 2;

    let json = serde_json::to_string(&score).unwrap();
    let deserialized: CurrentScore = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.runs, 150);
    assert_eq!(deserialized.wickets_lost, 3);
    assert_eq!(deserialized.over, 25);
    assert_eq!(deserialized.ball, 4);
    assert_eq!(deserialized.wides, 5);
    assert_eq!(deserialized.no_balls, 2);
}

#[test]
fn test_match_serialization() {
    let team1 = Team {
        name: "Team A".to_string(),
        players: vec![Player::new("Player1".to_string())],
    };
    let team2 = Team {
        name: "Team B".to_string(),
        players: vec![Player::new("Player2".to_string())],
    };

    let match_instance = Match::new(
        "M001".to_string(),
        "Test Match".to_string(),
        MatchType::T20,
        team1,
        team2,
    )
    .with_venue("Test Ground".to_string())
    .with_date("2025-01-01".to_string());

    let json = serde_json::to_string(&match_instance).unwrap();
    let deserialized: Match = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.id, "M001");
    assert_eq!(deserialized.title, "Test Match");
    assert_eq!(deserialized.venue, Some("Test Ground".to_string()));
    assert_eq!(deserialized.date, Some("2025-01-01".to_string()));
    assert!(matches!(deserialized.match_type, MatchType::T20));
    assert_eq!(deserialized.team1.name, "Team A");
    assert_eq!(deserialized.team2.name, "Team B");
}

#[test]
fn test_match_result_serialization() {
    let results = vec![
        MatchResult::Team1Won {
            margin: WinMargin::Runs(25),
            method: None,
        },
        MatchResult::Team2Won {
            margin: WinMargin::Wickets(4),
            method: None,
        },
        MatchResult::Team1Won {
            margin: WinMargin::Award,
            method: Some("forfeit".to_string()),
        },
        MatchResult::Tie { method: None },
        MatchResult::Draw,
        MatchResult::NoResult,
    ];

    for result in results {
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: MatchResult = serde_json::from_str(&json).unwrap();

        match (&result, &deserialized) {
            (
                MatchResult::Team1Won {
                    margin: WinMargin::Runs(r1),
                    ..
                },
                MatchResult::Team1Won {
                    margin: WinMargin::Runs(r2),
                    ..
                },
            ) => assert_eq!(r1, r2),
            (
                MatchResult::Team2Won {
                    margin: WinMargin::Wickets(w1),
                    ..
                },
                MatchResult::Team2Won {
                    margin: WinMargin::Wickets(w2),
                    ..
                },
            ) => assert_eq!(w1, w2),
            (
                MatchResult::Team1Won {
                    margin: WinMargin::Award,
                    ..
                },
                MatchResult::Team1Won {
                    margin: WinMargin::Award,
                    ..
                },
            ) => {} // Award margins match
            (MatchResult::Tie { .. }, MatchResult::Tie { .. }) => {}
            (MatchResult::Draw, MatchResult::Draw) => {}
            (MatchResult::NoResult, MatchResult::NoResult) => {}
            _ => panic!("Serialization/deserialization mismatch"),
        }
    }
}

#[test]
fn test_match_type_serialization() {
    let types = vec![
        MatchType::Test,
        MatchType::OD,
        MatchType::T20,
        MatchType::Other("The Hundred".to_string()),
    ];

    for match_type in types {
        let json = serde_json::to_string(&match_type).unwrap();
        let deserialized: MatchType = serde_json::from_str(&json).unwrap();

        match (&match_type, &deserialized) {
            (MatchType::Test, MatchType::Test) => {}
            (MatchType::OD, MatchType::OD) => {}
            (MatchType::T20, MatchType::T20) => {}
            (MatchType::Other(s1), MatchType::Other(s2)) => assert_eq!(s1, s2),
            _ => panic!("Match type serialization/deserialization mismatch"),
        }
    }
}

#[test]
fn test_wicket_serialization() {
    let wicket = Wicket {
        player_out: "Test Player".to_string(),
        kind: "bowled".to_string(),
    };

    let json = serde_json::to_string(&wicket).unwrap();
    let deserialized: Wicket = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.player_out, "Test Player");
    assert_eq!(deserialized.kind, "bowled");
}

#[test]
fn test_ball_events_serialization() {
    let events = vec![
        BallEvents::Four,
        BallEvents::Six,
        BallEvents::Bye(2),
        BallEvents::LegBye(1),
        BallEvents::Wide(1),
        BallEvents::NoBall(1),
        BallEvents::Penalty(5),
    ];

    for event in events {
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: BallEvents = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }
}

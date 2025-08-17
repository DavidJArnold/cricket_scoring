use std::char;

use cricket_scoring::error::BallString;
use cricket_scoring::scoring::ball::{BallEvents, BallOutcome, Wicket};
use cricket_scoring::scoring::player::Team;
use cricket_scoring::scoring::{innings::Innings, player::Player};

fn parse(ball: &str, on_strike: &Player, off_strike: &Player) -> Result<BallOutcome, BallString> {
    // basic format is runs followed by extra events:
    //   1: 1 run
    //   .: No run
    //   W: Wicket (no runs)
    //   1X: 1 wide (equivalent to X)
    //   4X: 4 wides
    //   WX: Wicket and wide
    //   4L: 4 leg byes
    //   N: New over
    //
    // dot -> . (equivalent to 0)
    // runs -> 0, 1, 2, 3, 4, etc.
    // wicket -> W
    // wide -> X
    // no ball -> O
    // bye -> B
    // leg bye -> L
    // four -> F
    // six -> S
    //
    // empty input is not permitted
    //
    // Records must be some digits or a period, followed by up to three of W/X/B/L/O/F/S, or N
    // Valid combinations: W, WX, WB, WL, WO, X, O, OB, OL, L, B, WOF, WOS, XF, OF, OS, OBF, OLF, LF, BF.
    //
    // If no period or digits are found, it will be assumed no runs were scored.
    // Therefore, a digit must appear with B or L to indicate how many byes/leg byes.
    //
    // TODO: Prevent duplicate letters/periods, verify ordering
    const ALLOWED_CHARS: [char; 8] = ['.', 'W', 'X', 'B', 'L', 'O', 'F', 'S'];

    let mut ball_events = vec![];

    if ball.is_empty() {
        return Err(BallString::EmptyBallString);
    }
    for c in ball.chars() {
        if !(char::is_ascii_digit(&c) || ALLOWED_CHARS.contains(&c)) {
            return Err(BallString::InvalidBallStringCharacter(c));
        }
    }

    if (ball.contains('B') || ball.contains('L')) && !ball.chars().next().unwrap().is_ascii_digit()
    {
        // A bye/leg bye must include the number of runs scored
        return Err(BallString::InvalidByeCharacter);
    }

    if (ball.contains('F') && ball.contains('S')) || (ball.contains('B') && ball.contains('L')) {
        // cannot have both a four and a six, or a bye and a leg bye
        return Err(BallString::InvalidBallDescription);
    }

    if ball.contains('W') {
        ball_events.push(BallEvents::Wicket(vec![Wicket {
            player_out: on_strike.name.clone(),
            kind: "unknown".to_string(),
        }]));
    } else if ball.contains('X') {
        ball_events.push(BallEvents::Wide(1));
    } else if ball.contains('O') {
        ball_events.push(BallEvents::NoBall(1));
    } else if ball.contains('L') {
        ball_events.push(BallEvents::LegBye(1));
    } else if ball.contains('B') {
        ball_events.push(BallEvents::Bye(1));
    } else if ball.contains('F') {
        ball_events.push(BallEvents::Four);
    } else if ball.contains('S') {
        ball_events.push(BallEvents::Six);
    };

    let runs = if ball.starts_with('.') {
        0
    } else {
        let runs_string = ball.matches(char::is_numeric).next();
        match runs_string {
            None => 0,
            Some(x) => x.parse::<i32>().expect("Can't convert to i32"),
        }
    };

    Ok(BallOutcome::new(
        runs,
        ball_events,
        on_strike.clone(),
        off_strike.clone(),
    ))
}

fn main() {
    let mut team: Vec<Player> = vec![];
    let a: char = 'A';
    for idx in 0..11 {
        team.push(Player::new(
            String::from_utf8(vec![u8::try_from(a as usize + idx).unwrap()]).unwrap(),
        ));
    }
    println!("{team:?}");
    let mut innings: Innings = Innings::new(
        Team {
            name: "Team A".to_string(),
            players: team.clone(),
        },
        Team {
            name: "Team B".to_string(),
            players: team.clone(),
        },
    );
    println!(". or digit for runs, W (wicket), X (wide), O (no ball), B (bye), L (leg bye), F (four), S (six), N (over)");
    loop {
        let mut ball_desc = String::new();
        let _ = std::io::stdin().read_line(&mut ball_desc);
        ball_desc = ball_desc.strip_suffix('\n').unwrap().to_string();
        if ball_desc == "Finished" {
            break;
        } else if ball_desc.len() == 1
            && ball_desc.chars().next().unwrap_or('x').to_ascii_uppercase() == 'N'
        {
            innings.over();
            println!(". or digit for runs, W (wicket), X (wide), O (no ball), B (bye), L (leg bye), F (four), S (six), N (over)");
        } else {
            let on_strike = innings.batting_team.players.get(innings.on_strike).unwrap();
            let off_strike = innings
                .batting_team
                .players
                .get(innings.off_strike)
                .unwrap();
            let ball_outcome =
                parse(&ball_desc.to_ascii_uppercase(), on_strike, off_strike).unwrap();
            ball_outcome.validate().unwrap();
            innings.score_ball(&ball_outcome);
            println!("{}", innings.score);
        }

        if innings.score.wickets_lost == 10 {
            break;
        }
    }
    println!("{innings}\n");
}

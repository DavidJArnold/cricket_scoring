use core::fmt;

#[derive(Default, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub runs: i32,
    pub balls_faced: i32,
    pub fours: i32,
    pub sixes: i32,
    pub out: bool,
}

pub type Team = [Player; 11];

impl Player {
    #[must_use]
    pub fn new(name: String) -> Self {
        Player {
            name,
            ..Default::default()
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut not_out_ind = "";
        if !self.out {
            not_out_ind = "*";
        }
        write!(
            f,
            "{}: {}{}({}), {} 4s, {} 6s",
            self.name, self.runs, not_out_ind, self.balls_faced, self.fours, self.sixes
        )
    }
}

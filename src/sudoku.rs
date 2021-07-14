use std::fmt;


#[derive(Debug)]
pub struct Game {
    pub name: String,
}

impl Game {
    pub fn new(game_name: &str, _initial: &'static str) -> Self {
        Self {
            name: game_name.to_string(),
        }
    }
}

// Default toString implementation. Does nothing now, will print current value of game, solution if found one
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::games::*;

    #[test]
    fn test_create_game_with_name() {
        let game = Game::new("easy", easy_sudoku());
        assert_eq!("easy", game.name);
    }
}

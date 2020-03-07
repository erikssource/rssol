mod card;
mod command;
mod deck;
mod foundation;
mod game;
mod pile;
mod results;
mod stock;
mod tableau;
mod waste;

pub use results::Success;
pub use results::Failure;

const HELP_TEXT: &str = 
  "-----------------------------------------------------\
  ? . . . . .: Display Help\
  r          : Retire Current Game\
  q . . . . .: Quit\
  n          : Draw Cards from Stock\
  a . . . . .: Try to Automatically Finish Game\
  k          : Move Card from Waste to Foundation\
  k[1-7] . . : Move Card from Waste to Pile by Number\
  [1-7]      : Move Card from Pile by Number to Foundation\
  [1-7][1-7].: Move Card from Pile to Pile by Number\
  h[1-7]     : Move Card from Hearts Foundation to Pile by Number\
  d[1-7] . . : Move Card from Diamonds Foundation to Pile by Number\
  s[1-7]     : Move Card from Spades Foundation to Pile by Number\
  c[1-7] . . : Move Card from Clubs Foundation to Pile by Number";

pub struct Solitaire {
  game: game::Game,
}

impl Solitaire {
  pub fn new() -> Solitaire {
    Solitaire{ game: game::Game::new() }
  }

  pub fn command(&mut self, cmd_str: &str) -> Result<results::Success, results::Failure> {
    match command::Command::from_string(cmd_str) {
      Some(cmd) => self.game.exec_command(cmd),
      None => Err(results::Failure::InvalidCommand)
    }
    
  }

  pub fn new_game(&mut self) {
    self.game = game::Game::new();
  }

  pub fn get_help() -> &'static str {
    HELP_TEXT
  }

  pub fn display(&self) -> String {
    self.game.display()
  }
}


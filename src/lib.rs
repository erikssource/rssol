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
  "-----------------------------------------------------\n\
  ? . . . . .: Display Help\n\
  r          : Retire Current Game\n\
  q . . . . .: Quit\n\
  n          : Draw Cards from Stock\n\
  a . . . . .: Try to Automatically Finish Game\n\
  k          : Move Card from Waste to Foundation\n\
  k[1-7] . . : Move Card from Waste to Pile by Number\n\
  [1-7]      : Move Card from Pile by Number to Foundation\n\
  [1-7][1-7].: Move Card from Pile to Pile by Number\n\
  h[1-7]     : Move Card from Hearts Foundation to Pile by Number\n\
  d[1-7] . . : Move Card from Diamonds Foundation to Pile by Number\n\
  s[1-7]     : Move Card from Spades Foundation to Pile by Number\n\
  c[1-7] . . : Move Card from Clubs Foundation to Pile by Number\n";

pub struct Solitaire {
  game: game::Game,
}

impl Solitaire {
  pub fn new() -> Solitaire {
    Solitaire{ game: game::Game::new() }
  }

  pub fn command(&mut self, cmd_str: &str) -> Result<results::Success, results::Failure> {
    match command::Command::from_string(cmd_str) {
      Some(cmd) => {
        match cmd {
          command::Command::Quit => Ok(results::Success::Quit),
          command::Command::ShowHelp => Ok(results::Success::Help(HELP_TEXT.to_string())),
          command::Command::Retire => {
            self.new_game();
            Ok(results::Success::Retire)
          }
          _ => self.game.exec_command(cmd)
        }        
      },
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


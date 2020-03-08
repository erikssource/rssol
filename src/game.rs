use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::card;
use crate::command;
use crate::foundation::Foundation;
use crate::stock::Stock;
use crate::tableau::Tableau;
use crate::results;
use crate::waste::Waste;
use crate::deck::FULL_DECK;

pub const HEART_FD: u8 = 0;
pub const DIAMOND_FD: u8 = 1;
pub const SPADE_FD: u8 = 2;
pub const CLUB_FD: u8 = 3;

pub struct Game {
  foundations: [Foundation; 4],
  stock: Stock,
  waste: Waste,
  tableau: Tableau,
  turn: u16,
}

impl Game {
  pub fn new() -> Game {
    let mut cards = Vec::new();
    for card in FULL_DECK.iter() {
      cards.push(card);
    }
    cards.shuffle(&mut thread_rng());
    // Create and populate tableau
    let mut tableau = Tableau::new();

    // Deal tableau
    for i in 0..7 {
      for _ in 0..=i {
        match cards.pop() {
          Some(card) => tableau.add_card(i, card),
          None => panic!("Not playing with a full deck"),
        }
      }
    }
    tableau.flip_all();

    Game{ 
      foundations: [
        Foundation::new(card::Suit::Heart),
        Foundation::new(card::Suit::Diamond),
        Foundation::new(card::Suit::Spade),
        Foundation::new(card::Suit::Club),
      ],
      stock: Stock::new(cards),
      waste: Waste::new(),
      tableau,
      turn: 0,
    }
  }

  pub fn exec_command(&mut self, cmd: command::Command) -> Result<results::Success, results::Failure> {
    match cmd {
      command::Command::DrawFromStock => {
        self.draw_from_stock()
      },
      command::Command::WasteToFoundation => {
        self.waste_to_foundation()
      },
      command::Command::AutoFinish => {
        self.auto_finish()
      },
      command::Command::WasteToPile{pile_index} => {
        self.waste_to_pile(pile_index)
      },
      command::Command::PileToFoundation{pile_index} => {
        self.pile_to_foundation(pile_index)
      },
      command::Command::PileToPile{src_pile, dest_pile} => {
        self.pile_to_pile(src_pile, dest_pile)
      },
      command::Command::FoundationToPile{foundation_index, pile_index} => {
        self.foundation_to_pile(foundation_index, pile_index)
      },
      _ => {
        Err(results::Failure::InvalidCommand)
      },
    }
  }

  pub fn victory(&self) -> bool {
    self.foundations[HEART_FD as usize].is_full() 
      && self.foundations[DIAMOND_FD as usize].is_full() 
      && self.foundations[SPADE_FD as usize].is_full() 
      && self.foundations[CLUB_FD as usize].is_full() 
  }

  pub fn success(&self) -> results::Success {
    match self.victory() {
      true => results::Success::Victory(self.display()),
      false => results::Success::ValidMove(self.display())
    }
  }

  pub fn draw_from_stock(&mut self) -> Result<results::Success, results::Failure> {
    if self.stock.is_empty() {
      self.stock.refresh(self.waste.get_all());
      self.waste.clear();
    }
    let opt = self.stock.take(1);
    match opt {
      Some(taken) => {
        self.waste.put(taken);
        self.turn += 1;
        Ok(self.success())
      },
      None => {
        Err(results::Failure::InvalidMove)
      }
    }
  }

  pub fn waste_to_foundation(&mut self) -> Result<results::Success, results::Failure> {
    let opt = self.waste.get_top();
    match opt {
      Some(top_card) => {
        for foundation in self.foundations.iter_mut() {
          if foundation.can_add(top_card) {
            let pop_opt = self.waste.take();
            if let Some(pop) = pop_opt {
              foundation.add(pop);
              self.turn += 1;
              return Ok(self.success());
            }
          }
        }
        Err(results::Failure::InvalidMove)
      },
      None => Err(results::Failure::InvalidMove),
    }
  }

  pub fn auto_finish(&mut self) -> Result<results::Success, results::Failure> {
    loop {
      let mut low_pile = 0;
      let mut low_value = card::Value::King.rank() + 1;
      for i in 0..7
      {
        let card_opt = self.tableau.piles[i].get_top();
        if let Some(card) = card_opt {
          if card.value.rank() < low_value {
            low_value = card.value.rank();
            low_pile = (i + 1) as u8;
          }
        }
      }
      if low_pile > 0 {
        let result = self.pile_to_foundation(low_pile);
        if let Err(_) = result {
          // Break from loop since we couldn't move the card.
          break;
        }
      }
      else {
        break;
      }
    }
    Ok(self.success())
  }

  pub fn waste_to_pile(&mut self, pile_num: u8) -> Result<results::Success, results::Failure> {
    let pile_idx = pile_num - 1;
    if let Some(top_card) = self.waste.get_top() {
      let pile = &mut self.tableau.piles[pile_idx as usize];
      if pile.can_add(top_card) {
        if let Some(popped_card) = self.waste.take() {
          pile.add_card(popped_card);
          self.turn += 1;
          return Ok(self.success());
        }
      }
    }
    Err(results::Failure::InvalidMove)
  }

  pub fn foundation_to_pile(&mut self, foundation_index: u8, pile_index: u8) -> Result<results::Success, results::Failure> {
    //TODO: Get rid of duplicate code with wast to pile. Probably need to define trait 
    //      for being able to take a card.
    if let Some(top_card) = self.foundations[foundation_index as usize].get_top() {
      let pile_idx = pile_index - 1;
      let pile = &mut self.tableau.piles[pile_idx as usize];
      if pile.can_add(top_card) {
        if let Some(popped_card) = self.foundations[foundation_index as usize].take() {
          pile.add_card(popped_card);
          self.turn += 1;
          return Ok(self.success());
        }
      }
    }
    Err(results::Failure::InvalidMove)
  }

  pub fn pile_to_foundation(&mut self, pile_num: u8) -> Result<results::Success, results::Failure> {
    match self.tableau.get_top(pile_num - 1) {
      Some(top_card) => {
        for foundation in self.foundations.iter_mut() {
          if foundation.can_add(top_card) {
            if let Some(pop) = self.tableau.take(pile_num - 1) {
              foundation.add(pop);
              self.turn += 1;
              return Ok(self.success());
            }
          }
        }        
        Err(results::Failure::InvalidMove)
      },
      None => {
        Err(results::Failure::InvalidMove)
      }
    }
  }

  pub fn pile_to_pile(&mut self, src_pile_num: u8, dest_pile_num: u8) -> Result<results::Success, results::Failure> {
    let src_idx = src_pile_num - 1;
    let dest_idx = dest_pile_num - 1;
    match self.tableau.do_move(src_idx, dest_idx) {
      Ok(_) => { 
        self.turn += 1;
        Ok(self.success())
      },
      Err(_) => {
        Err(results::Failure::InvalidMove)
      },
    }
  }

  pub fn display(&self) -> String {
    let mut display = "-----------------------------------------------------\n".to_owned();
    display.push_str(&format!("Turn: {}   Stock: {}   Waste: {}\n", self.turn, self.stock.size(), self.waste.size()));
    display.push_str("    n      k            h      d      s      c\n");
    display.push_str(&format!("  {}  {}        {}  {}  {}  {}\n\n",
      self.stock,
      self.waste,
      self.foundations[HEART_FD as usize],
      self.foundations[DIAMOND_FD as usize],
      self.foundations[SPADE_FD as usize],
      self.foundations[CLUB_FD as usize]
    ));
    display.push_str(&self.tableau.display());
    display
  }
}

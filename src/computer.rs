use crate::game::Game;

macro_rules! min {
  ($a:expr, $b:expr) => {
    if $a < $b { $a }
    else { $b }
  };
}

macro_rules! max {
  ($a:expr, $b:expr) => {
    if $a > $b { $a }
    else { $b }
  };
}

impl Game {

  pub fn find_best_move(&self) -> Option<Game> {
    let mut best_move = None;
    let mut best_score = -1000;
    for (row, col) in self.free_spaces() {
      let next = self.place_figure(row, col).unwrap();
      let result = next.minimax(false);
      if result > best_score {
        best_score = result;
        best_move = Some(next)
      }
    }
    best_move
  } 

  fn free_spaces(&self) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    for row in 0 .. 3 {
      for col in 0 .. 3 {
        if self.board[row][col] == None {
          output.push((row, col))
        }
      }
    }
    output
  }

  fn minimax(&self, maximising: bool) -> i32 {
    if self.status != None || self.free_space == 0 {
      return self.evaluate_finished_game()
    }
    if maximising { self.maximising() }
    else { self.minimising() }
  }

  fn maximising(&self) -> i32 {
    let mut best_score = -1000;
    for (row, col) in self.free_spaces() {
      let next = self.place_figure(row, col).unwrap();
      let score = next.minimax(false);
      best_score = max!(best_score, score);
    }
    best_score
  }

  fn minimising(&self) -> i32 {
    let mut worst_score = 1000;
    for (row, col) in self.free_spaces() {
      let next = self.place_figure(row, col).unwrap();
      let score = next.minimax(true);
      worst_score = min!(worst_score, score);
    }
    worst_score
  }

  fn evaluate_finished_game(&self) -> i32 {
    match self.status {
      None => 0,
      Some(true) => 1,
      _ => -1
    }
  }

}

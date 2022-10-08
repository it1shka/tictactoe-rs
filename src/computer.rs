use crate::game::Game;

impl Game {

  pub fn find_best_move(&self) -> Option<Game> {
    let mut best_move = None;
    let mut best_score = -1000;
    for (row, col) in self.free_spaces() {
      let next = self.place_figure(row, col).unwrap();
      let result = next.evaluate_game(-1);
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

  fn evaluate_game(&self, player: i32) -> i32 {
    if self.free_space == 0 || self.status != None {
      return self.evaluate_finished_game() * player
    }
    let mut best_score = -1000;
    for (row, col) in self.free_spaces() {
      let next = self.place_figure(row, col).unwrap();
      let result = next.evaluate_game(player * -1);
      if result > best_score {
        best_score = result
      }
    }
    best_score * player
  }

  fn evaluate_finished_game(&self) -> i32 {
    match self.status {
      None => 0,
      _ => 1,
    }
  }

}

#[derive(Clone, Copy)]
pub struct Game {
  pub board: [[Option<bool>; 3]; 3],
  // false = crosses,
  // true = zeroes
  pub turn: bool,
  // None = active
  // false = crosses won
  // true = zeroes won
  pub status: Option<bool>,
  pub free_space: usize
}

impl Game {
  pub fn new() -> Self {
    Game {
      board: [[None; 3]; 3],
      turn: false,
      status: None,
      free_space: 9
    }
  }

  pub fn display(&self) -> String {
    let mut output = String::new();
    for row in 0 .. 3 {
      for col in 0 .. 3 {
        output.push(match self.board[row][col] {
          None => '#',
          Some(false) => 'X',
          Some(true) => '0',
        })
      }
      output.push('\n')
    }
    let turn = 
      if self.turn { "Zero"  }
      else         { "Cross" };
    output.push_str(&format!("Current turn: {}\n", turn));
    output
  }

  pub fn display_game_result(&self) -> String {
    String::from(match self.status {
      None => "Draw!",
      Some(false) => "Crosses won!",
      Some(true) => "Zeroes won!"
    })
  }

  pub fn place_figure(&self, row: usize, col: usize) -> Result<Self, String> {
    if self.free_space == 0 {
      return Err(format!("Cannot place your figure: game is complete!"))
    }

    if !Self::valid_pos(row, col) {
      return Err(format!("Position ({}, {}) is invalid!", row, col))
    }

    if self.board[row][col] != None {
      return Err(format!("Position ({}, {}) is not empty!", row, col))
    }

    let mut next = *self;
    next.board[row][col] = Some(self.turn);
    next.turn = !self.turn;
    next.status = next.check_status();
    next.free_space -= 1;
    Ok(next)
  }

  fn check_status(&self) -> Option<bool> {
    let rows_to_check: Vec<Vec<(usize, usize)>> = vec![
      // rows
      vec![(0, 0), (0, 1), (0, 2)],
      vec![(1, 0), (1, 1), (1, 2)],
      vec![(2, 0), (2, 1), (2, 2)],

      // cols
      vec![(0, 0), (1, 0), (2, 0)],
      vec![(0, 1), (1, 1), (2, 1)],
      vec![(0, 2), (1, 2), (2, 2)],

      // diagonals
      vec![(0, 0), (1, 1), (2, 2)],
      vec![(2, 0), (1, 1), (0, 2)],
    ];

    for row in rows_to_check {
      let result = self.check_row(row);
      if result != None {
        return result
      }
    }
    None
  }

  fn check_row(&self, row: Vec<(usize, usize)>) -> Option<bool> {
    let (first_row, first_col) = row[0];
    let first_value = self.board[first_row][first_col];
    for (row, col) in row {
      if self.board[row][col] != first_value {
        return None
      }
    }
    first_value
  }

  #[inline(always)]
  fn valid_pos(row: usize, col: usize) -> bool {
    row < 3 && col < 3
  }
}
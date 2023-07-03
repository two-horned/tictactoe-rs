use std::fmt;

impl Game {
    pub fn new() -> Self {
        Self { turn: 0, history: [9;9], board: [0;9] }
    }

    pub fn from(turn: usize, history: [usize;9], board: [i8;9]) -> Self {
        Self { turn, history, board }
    }
    
    pub fn set_history(&mut self, history: [usize;9]) {
        self.history = history;
    }

    pub fn set_board(&mut self, board: [i8;9]) {
        self.board = board;
    }

    pub fn history_vec(&self) -> Vec<usize> {
        let mut v = vec![];
        for i in 0..self.turn {
            v.push(self.history[i]);
        }
        v
    }

    pub fn history_array(&self) -> [usize;9] {
        self.history.clone()
    }

    pub fn board(&self) -> [i8;9] {
        self.board.clone()
    }

    pub fn board_2d(&self) -> [[i8;3];3] {
        let mut b = [[0;3];3];
        for i in 0..9 {
            b[i/3][i%3] = self.board[i];
        }
        b
    }

    pub fn player(&self) -> i8 {
        if (self.turn % 2) != 0 {
            return -1;
        }
        1
    }

    pub fn pseudo_choose(&self, index: usize) -> bool {
        if 8 < index || self.board[index] != 0 { 
            return false;
        }
        return true;
    }

    pub fn choose(&mut self, index: usize) -> bool {
        if 8 < index || self.board[index] != 0 { 
            return false;
        }
        self.board[index] = self.player();
        self.history[self.turn] = index;
        self.turn += 1;
        true
    }

    pub fn loud_choose(&self, index: usize) -> Self {
        if 8 < index || self.board[index] != 0 {
            return *self;
        }
        let mut history = self.history;
        let mut board = self.board;
        history[self.turn] = index;
        board[index] = self.player();
        Self { turn: self.turn + 1, history, board }
    }

    pub fn showfree(&self) -> Vec<usize> {
        let mut v = vec![];
        for i in 0..9 {
            if self.board[i] == 0 { v.push(i) }
        }
        v
    }

    pub fn symmshowfree(&self) -> Vec<usize> {
        let mut v = self.showfree().to_vec();
        if self.board[0..3] == self.board[6..9] {
            v.retain(|&c| c < 6);
        }

        if self.board[0] == self.board[2] && self.board[3] == self.board[5] && self.board[6] == self.board[8] {
            v.retain(|&c| (c + 1) % 3 != 0);
        }

        if self.board[1] == self.board[3] && self.board[2] == self.board[6] && self.board[5] == self.board[7] {
            v.retain(|&c| (c != 3) && (c != 6) & (c != 7));
        }

        if self.board[1] == self.board[5] && self.board[3] == self.board[7] && self.board[0] == self.board[8] {
            v.retain(|&c| (c != 5) && (c != 7) & (c != 8));
        }
        v
    }

    pub fn whowon(&self) -> i8 {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        for i in 0..3 {
            for j in 0..3 {
                a += self.board[3*i+j];
                b += self.board[3*j+i];
            }
            if a < -2 || a > 2 {
                return a.signum();
            } 
            if b < -2 || b > 2 {
                return b.signum();
            }
            c += self.board[4*i];
            d += self.board[2+2*i];
            a = 0;
            b = 0;
        }
        if c < -2 || c > 2 {
            return c.signum();
        }
        if d < -2 || d > 2 {
            return d.signum();
        }
        0
    }

    pub fn is_finished(&self) -> bool {
        if self.turn > 8 || self.whowon() != 0 {
            return true;
        }
        false
    }

}

impl From<Vec<usize>> for Game {
    fn from(history_vec: Vec<usize>) -> Self {
        let mut history: [usize;9] = [9;9];
        let mut board: [i8;9] = [0;9];
        let mut turn = 0;
        for i in history_vec {
            history[turn] = i;
            board[i] = if turn % 2 != 0 { -1 } else { 1 };
            turn += 1;
        }
        Self { turn, history, board }
    }
}

impl From<[usize;9]> for Game {
    fn from(history: [usize;9]) -> Self {
        let mut board: [i8;9] = [0;9];
        let mut turn = 0;
        for i in history {
            if i > 8 { break; }
            board[i] = if turn % 2 != 0 { -1 } else { 1 };
            turn += 1;
        }
        Self { turn, history, board }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for i in 0..3 {
            for j in 0..3 {
                s += &ftos(self.board[3*i+j]).to_string();
            }
            s += "\n"
        }
        write!(f, "{}",s )
    }
}

fn ftos(field: i8) -> char {
    match field.signum() {
        0 => '.',
        1 => 'X',
        -1 => 'O',
        _ => unreachable!()
    }
}

#[derive(Debug,PartialEq, Eq, Clone,Copy)]
pub struct Game {
    turn: usize,
    history: [usize;9],
    board: [i8;9]
}

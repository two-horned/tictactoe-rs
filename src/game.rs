use std::fmt;

impl Game {
    pub fn new() -> Self {
        Self {
            turn: 0,
            history: [9; 9],
            board: [0; 9],
        }
    }

    pub fn from(turn: u8, history: [u8; 9], board: [i8; 9]) -> Self {
        Self {
            turn,
            history,
            board,
        }
    }

    pub fn hash_board(&self) -> usize {
        let mut hash = 0;
        for i in 0..9 {
            match self.board[i] {
                1 => hash |= 1,
                -1 => hash |= 2,
                _ => (),
            }
            hash <<= 2;
        }
        hash
    }

    pub fn set_history(&mut self, history: [u8; 9]) {
        self.history = history;
    }

    pub fn set_board(&mut self, board: [i8; 9]) {
        self.board = board;
    }

    pub fn history_vec(&self) -> Vec<u8> {
        self.history.into_iter().filter(|&x| x < 9).collect()
    }

    pub fn history_array(&self) -> [u8; 9] {
        self.history
    }

    pub fn board(&self) -> [i8; 9] {
        self.board
    }

    pub fn board_2d(&self) -> [[i8; 3]; 3] {
        let mut b = [[0; 3]; 3];
        for i in 0..9 {
            b[i / 3][i % 3] = self.board[i];
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
        index <= 8 && self.board[index] == 0
    }

    pub fn choose(&mut self, index: usize) -> bool {
        if 8 < index || self.board[index] != 0 {
            return false;
        }
        self.board[index] = self.player();
        self.history[self.turn as usize] = index as u8;
        self.turn += 1;
        true
    }

    pub fn loud_choose(&self, index: usize) -> Self {
        if 8 < index || self.board[index] != 0 {
            return *self;
        }
        let mut history = self.history;
        let mut board = self.board;
        history[self.turn as usize] = index as u8;
        board[index] = self.player();
        Self {
            turn: self.turn + 1,
            history,
            board,
        }
    }

    pub fn showfree(&self) -> u16 {
        let mut free = 0;
        let mut num = 1;
        for i in 0..9 {
            if self.board[i] == 0 {
                free |= num;
            }
            num <<= 1;
        }
        free
    }

    pub fn symmshowfree(&self) -> u16 {
        let mut free = self.showfree();
        if self.board[0..3] == self.board[6..9] {
            free &= 0x3F
        }

        if self.board[0] == self.board[2]
            && self.board[3] == self.board[5]
            && self.board[6] == self.board[8]
        {
            free &= 0xDB;
        }

        if self.board[1] == self.board[3]
            && self.board[2] == self.board[6]
            && self.board[5] == self.board[7]
        {
            free &= 0x137;
        }

        if self.board[1] == self.board[5]
            && self.board[3] == self.board[7]
            && self.board[0] == self.board[8]
        {
            free &= 0x5F;
        }
        free
    }

    pub fn whowon(&self) -> i8 {
        let mut a;
        let mut b;
        let mut c = 0;
        let mut d = 0;
        for i in 0..3 {
            a = 0;
            b = 0;
            for j in 0..3 {
                a += self.board[3 * i + j];
                b += self.board[3 * j + i];
            }
            if a < -2 || a > 2 {
                return a.signum();
            }
            if b < -2 || b > 2 {
                return b.signum();
            }
            c += self.board[4 * i];
            d += self.board[2 + 2 * i];
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
        8 < self.turn || self.whowon() != 0
    }
}

impl From<Vec<usize>> for Game {
    fn from(history_vec: Vec<usize>) -> Self {
        let mut history = [9; 9];
        let mut board = [0; 9];
        let mut turn = 0;
        for i in history_vec {
            history[turn as usize] = i as u8;
            board[i] = if turn % 2 != 0 { -1 } else { 1 };
            turn += 1;
        }
        Self {
            turn,
            history,
            board,
        }
    }
}

impl From<[u8; 9]> for Game {
    fn from(history: [u8; 9]) -> Self {
        let mut board: [i8; 9] = [0; 9];
        let mut turn = 0;
        for i in history {
            if i > 8 {
                break;
            }
            board[i as usize] = if turn % 2 != 0 { -1 } else { 1 };
            turn += 1;
        }
        Self {
            turn,
            history,
            board,
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for i in 0..3 {
            for j in 0..3 {
                s += &ftos(self.board[3 * i + j]).to_string();
            }
            s += "\n"
        }
        write!(f, "{}", s)
    }
}

fn ftos(field: i8) -> char {
    match field.signum() {
        0 => '.',
        1 => 'X',
        -1 => 'O',
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Game {
    turn: u8,
    history: [u8; 9],
    board: [i8; 9],
}

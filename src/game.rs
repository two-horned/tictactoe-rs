use std::fmt;

impl Game {
    pub fn new() -> Self {
        let mut tmp = Self {
            turn: 0,
            whowon: 0,
            history: [9; 9],
            player: [0; 2],
        };
        tmp.update_whowon();
        tmp
    }

    pub fn history_vec(&self) -> Vec<u8> {
        self.history.into_iter().filter(|&x| x < 9).collect()
    }

    pub fn history_array(&self) -> [u8; 9] {
        self.history
    }

    pub fn player(&self) -> isize {
        1 - ((self.turn as isize & 1) << 1)
    }

    pub fn turn(&self) -> usize {
        self.turn as usize
    }

    pub fn pseudo_choose(&self, index: usize) -> bool {
        let mask = 1 << index;
        let occupied = self.player[0] | self.player[1];
        index <= 8 && occupied & mask == 0
    }

    pub fn choose(&mut self, index: usize) -> bool {
        if !self.pseudo_choose(index) {
            return false;
        }
        self.unsafe_choose(index);
        true
    }

    pub fn unsafe_choose(&mut self, index: usize) {
        self.player[(self.turn & 1) as usize] |= 1 << index;
        self.history[self.turn as usize] = index as u8;
        self.update_whowon();
        self.turn += 1;
    }

    pub fn showfree(&self) -> u16 {
        if !self.is_finished() {
            !(self.player[0] | self.player[1])
        } else {
            0
        }
    }

    pub fn symmshowfree(&self) -> u16 {
        let mut free = self.showfree();
        if self.player[0] >> 6 == 0x7 & self.player[0]
            && self.player[1] >> 6 == 0x7 & self.player[1]
        {
            free &= 0x3F
        }

        if (0x124 & self.player[0]) >> 2 == 0x49 & self.player[0]
            && (0x124 & self.player[1]) >> 2 == 0x49 & self.player[1]
        {
            free &= 0xDB;
        }

        if (self.player[0] & 0x88) >> 2 == self.player[0] & 0x22
            && (self.player[0] & 0x40) >> 4 == self.player[0] & 0x04
            && (self.player[1] & 0x88) >> 2 == self.player[1] & 0x22
            && (self.player[1] & 0x40) >> 4 == self.player[1] & 0x04
        {
            free &= 0x137;
        }

        if (self.player[0] & 0xA0) >> 4 == self.player[0] & 0x0A
            && (self.player[0] & 0x100) >> 8 == self.player[0] & 0x01
            && (self.player[1] & 0xA0) >> 4 == self.player[1] & 0x0A
            && (self.player[1] & 0x100) >> 8 == self.player[1] & 0x01
        {
            free &= 0x5F;
        }
        free
    }

    fn update_whowon(&mut self) {
        for (p, s) in self.player.into_iter().zip([1, -1]) {
            let mut num;
            num = p;
            for _ in 0..3 {
                if 0x07 & num == 0x07 {
                    self.whowon = s;
                    return;
                }
                num >>= 3;
            }

            num = p;
            for _ in 0..3 {
                if 0x49 & num == 0x49 {
                    self.whowon = s;
                    return;
                }
                num >>= 1;
            }

            if 0x111 & p == 0x111 || 0x54 & p == 0x54 {
                self.whowon = s;
                return;
            }
        }
        self.whowon = 0;
    }

    pub fn whowon(&self) -> isize {
        self.whowon as isize
    }

    pub fn is_finished(&self) -> bool {
        self.whowon() != 0 || 9 == self.turn
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(11);
        for i in 0..3 {
            for j in 0..3 {
                let mask = 1 << (i * 3 + j);
                if self.player[0] & mask != 0 {
                    s += "X";
                } else if self.player[1] & mask != 0 {
                    s += "O";
                } else {
                    s += ".";
                }
            }
            s += "\n"
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Game {
    turn: u8,
    whowon: i8,
    history: [u8; 9],
    player: [u16; 2],
}

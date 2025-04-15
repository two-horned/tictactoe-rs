use std::fmt;

impl Game {
    pub fn new() -> Self {
        Self {
            turn: 0,
            whowon: 0,
            player: [0; 2],
        }
    }

    pub fn player(&self) -> isize {
        1 - ((self.turn as isize) << 1)
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
        self.player[self.turn as usize] |= 1 << index;
        self.turn = 1 - self.turn;
        self.update_whowon();
    }

    pub fn unsafe_unchoose(&mut self, index: usize) {
        self.turn = 1 - self.turn;
        self.player[self.turn as usize] ^= 1 << index;
        self.update_whowon();
    }

    pub fn showfree(&self) -> u16 {
        if self.is_finished() {
            return 0;
        }
        0x1FF & !(self.player[0] | self.player[1])
    }

    pub fn symmshowfree(&self) -> u16 {
        if self.is_finished() {
            return 0;
        }
        let mut free = 0x1FF & !(self.player[0] | self.player[1]);
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
            let mut num = p;
            for _ in 0..3 {
                if test(num, 7) {
                    self.whowon = s;
                    return;
                }
                num >>= 3;
            }

            let mut num = p;
            for _ in 0..3 {
                if test(num, 0x49) {
                    self.whowon = s;
                    return;
                }
                num >>= 1;
            }

            if test(p, 0x111) || test(p, 0x54) {
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
        self.whowon() != 0 || test(self.player[0] | self.player[1], 0x1FF)
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

fn test(num: u16, mask: u16) -> bool {
    num & mask == mask
}

#[derive(Debug)]
pub struct Game {
    turn: u8,
    whowon: i8,
    player: [u16; 2],
}

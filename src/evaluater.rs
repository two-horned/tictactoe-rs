use std::collections::HashMap;
use crate::game::Game;

impl Evaluater {
    pub fn new() -> Evaluater {
        Self { evaluated: HashMap::new() }
    }

    fn minmax(&mut self, player: i8, list: &Vec<Game>) -> Game {
        let mut n = self.eval2(list.first().unwrap());
        let mut g;
        if n.whowon() == player {
            return n;
        }
        if player > 0 {
            for i in list.iter().skip(1) {
                g = self.eval2(i);
                if g.whowon() == 1 {
                    return g;
                }
                if n.whowon() < g.whowon() {
                    n = g;
                }
            }
        } else {
            for i in list.iter().skip(1) {
                g = self.eval2(i);
                if g.whowon() == -1 {
                    return g;
                }
                if n.whowon() > g.whowon() {
                    n = g;
                }
            }
        }
        n
    }

    pub fn eval2(&mut self, game: &Game) -> Game {
        if game.is_finished() {
            return game.clone();
        }
        let board = game.board();
        if self.evaluated.contains_key(&board) {
            let mut n = *self.evaluated.get(&board).unwrap();
            let gh = {
                let mut gh = game.history_array();
                let nh = n.history_array();
                for i in 0..9 { if gh[i] == 9 { gh[i] = nh[i] } }
                gh
            };
            n.set_history(gh);
            return n;
        }
        let free = game.symmshowfree();
        let mut v = vec![];
        let mut g;
        for i in free {
            g = game.loud_choose(i);
            if g.whowon() != 0 {
                self.evaluated.insert(board, g);
                return g;
            }
            v.push(g);
        }
        g = self.minmax(game.player(),&v);
        self.evaluated.insert(board, g);
        return g;
    }

    pub fn eval(&mut self, game: &Game) -> Game {
        if game.is_finished() {
            return game.clone();
        }
        let board = game.board();
        if self.evaluated.contains_key(&board) {
            let mut n = *self.evaluated.get(&board).unwrap();
            let gh = {
                let mut gh = game.history_array();
                let nh = n.history_array();
                for i in 0..9 { if gh[i] == 9 { gh[i] = nh[i] } }
                gh
            };
            n.set_history(gh);
            return n;
        }
        let free = game.symmshowfree();
        let player = game.player();
        let mut g;
        let mut n = game.loud_choose(free[0]);
        if n.whowon() != 0 {
            self.evaluated.insert(board, n);
            return n;
        }
        n = self.eval(&n);
        if n.whowon() == player {
            self.evaluated.insert(board, n);
            return n;
        }
        if player > 0 {
            for i in free.iter().skip(1) {
                g = game.loud_choose(*i);
                if g.whowon() != 0 {
                    self.evaluated.insert(board, n);
                    return g;
                }
                g = self.eval(&g);
                if g.whowon() == 1 {
                    n = g;
                    break;
                }
                if g.whowon() > n.whowon() {
                    n = g;
                }
            }
        } else {
            for i in free.iter().skip(1) {
                g = game.loud_choose(*i);
                if g.whowon() != 0 {
                    self.evaluated.insert(board, n);
                    return g;
                }
                g = self.eval(&g);
                if g.whowon() == -1 {
                    n = g;
                    break;
                }
                if g.whowon() < n.whowon() {
                    n = g;
                }
            }
        }
        self.evaluated.insert(board, n);
        n
    }
}

pub struct Evaluater {
    evaluated: HashMap<[i8;9],Game>
}

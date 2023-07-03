use std::collections::HashMap;
use crate::game::Game;

impl Evaluater {
    pub fn new() -> Evaluater {
        Self { evaluated: HashMap::new() }
    }

    fn minmax(&mut self, player: i8, list: &Vec<Game>) -> Game {
        let mut n = self.eval(list.first().unwrap());
        let mut g;
        if n.whowon() == player {
            return n;
        }
        if player > 0 {
            for i in list.iter().skip(1) {
                g = self.eval(i);
                if g.whowon() == 1 {
                    return g;
                }
                if n.whowon() < g.whowon() {
                    n = g;
                }
            }
        } else {
            for i in list.iter().skip(1) {
                g = self.eval(i);
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

    pub fn eval(&mut self, game: &Game) -> Game {
        if game.is_finished() {
            return game.clone();
        }
        let board = game.board();
        if self.evaluated.contains_key(&board) {
            let n = self.evaluated.get(&board).unwrap();
            let nh = n.history_array();
            let mut gh = game.history_array();
            for i in 0..9 { if gh[i] > 8 { gh[i] = nh[i] } }
            return Game::from(9, gh, n.board());
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
}

pub struct Evaluater {
    evaluated: HashMap<[i8;9],Game>
}

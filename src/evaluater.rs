use crate::game::Game;

pub fn eval(game: Game) -> Game {
    let mut free = game.symmshowfree();
    let mut best: Option<Game> = None;
    let player = game.player();

    for i in 0..9 {
        if free & 1 != 0 {
            let mut n = game;
            n.unsafe_choose(i);
            let n = eval(n);
            match best {
                Some(x) if n.whowon() * player <= x.whowon() * player => (),
                _ => best = Some(n),
            }
            if n.whowon() == game.player() {
                break;
            }
        }
        free >>= 1;
    }
    let g = match best {
        Some(x) => x,
        _ => game,
    };
    g
}

use crate::game::Game;

pub fn eval(game: &mut Game) -> EvalRes {
    let mut free = game.symmshowfree();
    let player = game.player();
    let mut best = EvalRes {
        score: game.whowon(),
        action: 9,
    };

    while free != 0 {
        let i = free.trailing_zeros() as usize;
        game.unsafe_choose(i);
        let n = eval(game);
        game.unsafe_unchoose(i);

        if best.action > 8 || best.score * player < n.score * player {
            best = EvalRes {
                score: n.score,
                action: i,
            }
        }
        if n.score == player {
            break;
        }
        free &= free - 1;
    }
    best
}

#[derive(Clone, Copy)]
pub struct EvalRes {
    pub score: isize,
    pub action: usize,
}

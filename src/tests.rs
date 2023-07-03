use crate::{evaluater::Evaluater,game::Game};

#[test]
fn test_symmshowfree() {
    assert_eq!(vec![0,1,4], Game::new().symmshowfree());
}


#[test]
fn test_choose() {
    let mut g = Game::new();
    let b = g.choose(0);
    assert_eq!([0,9,9,9,9,9,9,9,9], g.history_array());
    assert_eq!([1,0,0,0,0,0,0,0,0], g.board());
    assert_eq!(-1, g.player());
    assert_eq!(true, b);
}

#[test]
fn test_history() {
    let mut g = Game::new();
    for i in [0, 1, 6, 4, 3] {
        g.choose(i);
    }
    assert_eq!(vec![0,1,6,4,3], g.history_vec());
}

#[test]
fn test_whowon() {
    let mut g = Game::new();
    for i in [0, 1, 6, 4, 3] {
        g.choose(i);
    }
    assert_eq!(1, g.whowon());
}

#[test]
fn test_finished() {
    let mut g = Game::new();
    for i in [0, 1, 6, 4, 3] {
        g.choose(i);
    }
    assert_eq!(true, g.is_finished());
}

#[test]
fn final_test() {
    let mut ev = Evaluater::new();
    let g = ev.eval(&Game::new());
    assert_eq!([0, 4, 1, 2, 6, 3, 5, 7, 8],g.history_array());
}

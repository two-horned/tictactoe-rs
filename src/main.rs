use std::env::args;
use std::time::Instant;
use std::io::{self,Write};
use tictactoe::game::Game;
use tictactoe::evaluater::Evaluater;

fn player() -> io::Result<i8> {
    let mut e;
    loop {
        print!("Enter player [1,2]: ");
        io::stdout().flush()?;
        e = String::new();
        io::stdin().read_line(&mut e)?;
        match e.trim() {
            "q" => return Ok(0),
            "1" => return Ok(1),
            "2" => return Ok(-1),
            _ => (),
        }
    }
}

fn botting() -> io::Result<i8> {
    let mut e;
    loop {
        print!("Play with bot? [y,N]: ");
        io::stdout().flush()?;
        e = String::new();
        io::stdin().read_line(&mut e)?;
        match e.trim() {
            "q" => return Ok(0),
            "y" => return Ok(1),
            "N" => return Ok(-1),
            _ => ()
        }
    }
}

fn player_play(game: &Game) -> io::Result<usize> {
    let mut r = 9;
    let mut e;
    while !game.pseudo_choose(r) {
        print!("Turn of human player: ");
        io::stdout().flush()?;
        e = String::new();
        io::stdin().read_line(&mut e)?;
        match e.trim() {
            "q" => return Ok(9),
            "1" => r = 0,
            "2" => r = 1,
            "3" => r = 2,
            "4" => r = 3,
            "5" => r = 4,
            "6" => r = 5,
            "7" => r = 6,
            "8" => r = 7,
            "9" => r = 8,
            _ => ()
        }
    }
    Ok(r)
}

fn bot_play(game: &Game, evaluater: &mut Evaluater) -> io::Result<usize> {
    println!("Turn of bot");
    let gh = game.history_array();
    let nh = evaluater.eval(game).history_array();
    println!("{:?}",nh);
    for i in 0..9 {
        if gh[i] == 9 {
            return Ok(nh[i])
        }
    }
    unreachable!()
}

fn bench() {
    let mut evaluater = Evaluater::new();
    let g = Game::new();
    let s = Instant::now();
    evaluater.eval(&g);
    let s = s.elapsed();
    println!("Time needed to evaluate whole game tree: {}µs", s.as_micros());
}
fn main() -> io::Result<()> {
    let arg = args().nth(1);
    match arg {
        None => (),
        Some(s) => if s == "bench" {
            return Ok(bench());
        }
    }

    let mut evaluater = Evaluater::new();
    println!("Enter 'q' to quit");

    let p = player()?;
    if p == 0 { return Ok(()) };

    let b = botting()?;
    if b == 0 { return Ok(()) };
    let b = b == -1;

    let mut g = Game::new();
    let mut e;

    while !g.is_finished() {
        if g.player() == p || b {
            e = player_play(&g)?;
            if e == 9 { return Ok(()) };
        } else {
            e = bot_play(&g,&mut evaluater)?;
        }
        println!("Choice is: {}", e+1);
        g.choose(e);
        println!("{}", &g);
        if g.whowon() != 0 {
            break;
        }
    }

    Ok(())
}

extern crate getopts;
use getopts::Options;

extern crate rand;
use rand::Rng;

extern crate ncurses;

use std::env;
use std::{thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("w", "", "width of the board", "WIDTH");
    opts.optopt("t", "", "time in ms to wait in between generations", "TIME");
    opts.optopt("g", "", "number of generations", "NUM");
    opts.optflag("h", "help", "print help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        let u = opts.usage(format!("usage: {} [options]", program).as_str());
        println!("{}", u);
        return;
    }

    let num_gen: u64 = match matches.opt_str("g") {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(e) => panic!(e.to_string()),
        },
        None => 100,
    };

    let width: usize = match matches.opt_str("w") {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(e) => panic!(e.to_string()),
        },
        None => 60,
    };
    let height = width / 2;

    let time: u64 = match matches.opt_str("t") {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(e) => panic!(e.to_string()),
        },
        None => 500,
    };

    let mut board = vec![false; width * height];
    println!("num gens: {}", num_gen);
    println!("width:    {} cells", width);
    println!("height:   {} cells", height);
    println!("ms/gen:   {} ms", time);

    let mut rng = rand::thread_rng();
    let p = 1f64 / 5f64;
    for y in 0..height {
        for x in 0..width {
            board[idx(x, y, width)] = rng.gen_bool(p)
        }
    }

    println!("initial state:");
    println!("{}", screen(&board, width, height));

    ncurses::setlocale(ncurses::LcCategory::all, "");
    ncurses::initscr();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    for i in 0..num_gen {
        ncurses::erase();

        let new = step(board, width, height);
        board = new;
        ncurses::addstr(format!("gen {}\n", i).as_str());
        ncurses::addstr(screen(&board, width, height).as_str());

        if i != (num_gen - 1) {
            thread::sleep(time::Duration::from_millis(time));
        }

        ncurses::refresh();
    }

    ncurses::endwin();

    println!("final state:");
    println!("{}", screen(&board, width, height));
}

fn step(board: Vec<bool>, w: usize, h: usize) -> Vec<bool> {
    let mut new_board = board.clone();
    let mut nbrs = [false; 8];
    for y in 0..h {
        let up = y.checked_sub(1).unwrap_or(h - 1);
        let down = match y + 1 {
            d if d == h => 0,
            d => d,
        };
        for x in 0..w {
            let l = x.checked_sub(1).unwrap_or(w - 1);
            let r = match x + 1 {
                r if r == w => 0,
                r => r,
            };

            nbrs[0] = board[idx(l, up, w)];
            nbrs[1] = board[idx(x, up, w)];
            nbrs[2] = board[idx(r, up, w)];
            nbrs[3] = board[idx(l, y, w)];
            nbrs[4] = board[idx(r, y, w)];
            nbrs[5] = board[idx(l, down, w)];
            nbrs[6] = board[idx(x, down, w)];
            nbrs[7] = board[idx(r, down, w)];

            let mut n = 0u8;
            for b in nbrs.iter() {
                if *b {
                    n = n + 1;
                    if n > 3 {
                        break;
                    }
                }
            }

            let i = idx(x, y, w);
            new_board[i] = board[i];
            if board[i] {
                match n {
                    // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                    0..=1 => {
                        new_board[i] = false;
                    }
                    // 2. Any live cell with two or three live neighbours lives on to the next generation.
                    2..=3 => {
                        new_board[i] = true;
                    }
                    // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
                    _ => {
                        new_board[i] = false;
                    }
                };
                continue;
            }

            // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
            if n == 3 {
                new_board[i] = true;
            }
        }
    }

    new_board
}

fn screen(b: &Vec<bool>, w: usize, h: usize) -> String {
    let mut s = String::with_capacity((w * h) + h);
    for y in 0..h {
        for x in 0..w {
            if b[idx(x, y, w)] {
                s.push('\u{2588}');
            } else {
                s.push('\u{2591}');
            }
        }
        s.push('\n');
    }

    s
}

fn idx(x: usize, y: usize, w: usize) -> usize {
    (y * w) + x
}

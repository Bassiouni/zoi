mod env;
mod game_tree;

use std::env::consts::OS;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::{io, process::Command};

use env::Env;
use game_tree::GameTree;

fn main() {
    println!("Welcome to Tic Tac Toe Game!");
    println!("1. Play");
    println!("2. Quit");
    print!(">> ");
    #[allow(unused_must_use)]
    io::Write::flush(&mut io::stdout()).unwrap();

    let n = parse_input_to_i8();
    match n {
        1 => game_loop(),
        2 => exit(0),
        _ => exit(-1),
    };
}

fn game_loop() -> ! {
    let mut env = Env::new();
    clear_console();

    env.print_board();
    print!("select block to play at -> ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let n = parse_input_to_i8();
    sleep(Duration::from_millis(500));
    env.play(n);
    clear_console();
    env.print_board();

    let mut gt = GameTree::from(&env);
    gt.gen_tree();
    gt.eval_score();
    gt.update_env(&mut env);
    sleep(Duration::from_millis(500));

    loop {
        clear_console();
        env.print_board();
        sleep(Duration::from_millis(500));
        if let Some(val) = env.eval_winner() {
            println!("Winner Is Player {val}");
            exit(0);
        }

        if env.is_full() {
            println!("Tie!");
        }
        print!("select block to play at -> ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let n = parse_input_to_i8();
        env.play(n);
        clear_console();
        env.print_board();

        if let Some(val) = env.eval_winner() {
            println!("Winner Is Player {val}");
            exit(0);
        }

        if env.is_full() {
            println!("Tie!");
            exit(0);
        }

        sleep(Duration::from_millis(500));
        gt.update_env(&mut env);
    }
}

fn clear_console() {
    if OS == "linux" {
        Command::new("clear").status().unwrap();
    } else if OS == "windows" {
        Command::new("cls").status().unwrap();
    }
}

fn parse_input_to_i8() -> i8 {
    let mut buf = String::new();
    if let Err(err) = io::stdin().read_line(&mut buf) {
        println!("error {err}");
        exit(2);
    }

    buf.pop();
    match buf.parse::<i8>() {
        Ok(res) => res,
        Err(err) => {
            println!("error {err}");
            exit(2);
        }
    }
}

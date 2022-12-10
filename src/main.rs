mod env;
mod game_tree;

use env::{Env, State};
use game_tree::GameTree;

fn main() {
    let mut env = Env::new();
    env.insert_state_at(2, 1, State::X);
    env.print_board();
    match env.eval_winner() {
        Some(val) => println!("Winner Is Player {val}"),
        None => println!("No Winner!"),
    }

    let mut gt = GameTree::from(&env);
    gt.gen_tree();
}

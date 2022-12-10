use super::Node;
use crate::env::Env;

pub struct GameTree {
    head: Node,
}

impl GameTree {
    pub fn new() -> Self {
        GameTree { head: Node::new() }
    }

    pub fn from(env: &Env) -> Self {
        GameTree {
            head: Node::from(&env),
        }
    }

    pub fn gen_tree(&mut self) {
        self.head.construct_children();
    }

    pub fn minimax_eval() { todo!() }
}

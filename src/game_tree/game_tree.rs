extern crate traversal;

use traversal::bft;

use super::Node;
use crate::env::Env;

pub struct GameTree {
    head: Node,
}

impl GameTree {
    /// Creates a new [`GameTree`].
    pub fn new() -> Self {
        GameTree { head: Node::new() }
    }

    /// Generate a new [`GameTree`] from existing [`Env`]
    pub fn from(env: &Env) -> Self {
        GameTree {
            head: Node::from(env),
        }
    }

    pub fn gen_tree(&mut self) {
        self.head.construct_children();
    }

    pub fn eval_score(&mut self) {
        for node in self.head.children_mut().iter_mut() {
            Self::dfs_traverse_tree(node);
        }
    }

    fn dfs_traverse_tree(node: &mut Node) {
        if node.children().is_empty() {
            return;
        }

        for n in node.children_mut().iter_mut() {
            Self::dfs_traverse_tree(n);

            n.set_score(Self::calc_score_value(n));
        }
    }

    fn calc_score_value(node: &Node) -> i64 {
        if !node.children().is_empty() {
            let mut score_value: i64 = 0;
            for i in node.children().iter() {
                score_value += i.score();
            }

            return score_value;
        }
        return *node.score();
    }

    pub fn update_env(&mut self, env: &mut Env) {
        let tree_iter = bft(&self.head, |n| n.children().iter());
        let mut it: Node = Node::new();
        for item in tree_iter {
            if item.1.env() == env {
                it = item.1.to_owned();
            }
        }
        self.head = it;

        let new_move = self
            .head
            .children()
            .iter()
            .min_by_key(|p| p.score())
            .unwrap();
        env.set_env(new_move.env());
    }
}

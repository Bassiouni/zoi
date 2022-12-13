use super::Node;
use crate::env::Env;
use std::collections::VecDeque;

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
            println!("{}", n.score());
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
        let mut deq: VecDeque<&Node> = VecDeque::new();
        deq.push_back(&self.head);
        'l: loop {
            let front = *deq.front().unwrap();

            if front.env() == env {
                self.head = front.clone();
                break 'l;
            }
            
            if deq.len() == 1 {
                for i in front.children() {
                    deq.push_back(i);
                }
            }
            
            deq.pop_front();
        }
        let new_move = self.head.children().iter().min_by_key(|p| p.score()).unwrap();
        env.set_env(new_move.env());
    }
}

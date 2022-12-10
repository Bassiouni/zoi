use std::i8;

use crate::env::{Env, State};

pub struct Node {
    env: Env,
    score: u8,
    children: Vec<Self>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            score: 0,
            env: Env::new(),
            children: vec![],
        }
    }

    pub fn from(env: &Env) -> Self {
        Self {
            score: 0,
            env: Env::from(&env),
            children: vec![],
        }
    }

    pub fn construct_children(&mut self) {
        let ground = self.env.ground();

        let mut i = 0;
        while i < 3 {
            let mut j = 0;
            while j < 3 {
                if ground[i][j] == State::Empty {
                    let mut new_env = Env::from(&self.env);
                    new_env.play(i, j);
                    self.children.push(Self::from(&new_env));
                }
                j += 1;
            }
            i += 1;
        }

        for c in self.children.iter_mut() {
            if let Some(winner) = c.env.eval_winner() {
                c.score = (*winner) as u8;
            } else {
                c.construct_children();
            }
        }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn score(&self) -> &u8 {
        &self.score
    }

    pub fn set_score(&mut self, score: u8) {
        self.score = score;
    }

    pub fn children(&self) -> &Vec<Self> {
        &self.children
    }
}

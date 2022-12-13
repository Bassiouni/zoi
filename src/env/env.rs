use super::state::State;

type Ground = [[State; 3]; 3];

#[derive(Clone, Copy, PartialEq)]
pub struct Env {
    ground: Ground,
    last_played_state: State,
}

impl Env {
    pub fn new() -> Self {
        Env {
            ground: [
                [State::Empty, State::Empty, State::Empty],
                [State::Empty, State::Empty, State::Empty],
                [State::Empty, State::Empty, State::Empty],
            ],
            last_played_state: State::Empty,
        }
    }

    pub fn from(env: &Env) -> Self {
        let mut ret: Ground = Self::new().ground;
        let mut i = 0;
        while i < 3 {
            let mut j = 0;
            while j < 3 {
                ret[i][j] = env.ground[i][j];
                j += 1;
            }
            i += 1;
        }
        Self {
            ground: ret,
            last_played_state: env.last_played_state,
        }
    }

    /// Sets the env of this [`Env`].
    pub fn set_env(&mut self, env: &Env) {
        self.ground = env.ground;
        self.last_played_state = env.last_played_state;
    }

    pub fn eval_winner(&self) -> Option<&State> {
        // row matching
        if self.ground[0][0] == self.ground[0][1]
            && self.ground[0][1] == self.ground[0][2]
            && self.ground[0][0] != State::Empty
        {
            return Some(&self.ground[0][0]);
        }

        if self.ground[1][0] == self.ground[1][1]
            && self.ground[1][1] == self.ground[1][2]
            && self.ground[1][0] != State::Empty
        {
            return Some(&self.ground[1][0]);
        }

        if self.ground[2][0] == self.ground[2][1]
            && self.ground[2][1] == self.ground[2][2]
            && self.ground[2][0] != State::Empty
        {
            return Some(&self.ground[2][0]);
        }

        // col matching
        if self.ground[0][0] == self.ground[1][0]
            && self.ground[1][0] == self.ground[2][0]
            && self.ground[0][0] != State::Empty
        {
            return Some(&self.ground[0][0]);
        }

        if self.ground[0][1] == self.ground[1][1]
            && self.ground[1][1] == self.ground[2][1]
            && self.ground[0][1] != State::Empty
        {
            return Some(&self.ground[0][1]);
        }

        if self.ground[0][2] == self.ground[1][2]
            && self.ground[1][2] == self.ground[2][2]
            && self.ground[0][2] != State::Empty
        {
            return Some(&self.ground[0][2]);
        }

        // diagonal matching
        if self.ground[0][0] == self.ground[1][1]
            && self.ground[1][1] == self.ground[2][2]
            && self.ground[0][0] != State::Empty
        {
            return Some(&self.ground[0][0]);
        }

        // reverse diagonal matching
        if self.ground[2][0] == self.ground[1][1]
            && self.ground[1][1] == self.ground[0][2]
            && self.ground[2][0] != State::Empty
        {
            return Some(&self.ground[2][0]);
        }

        None
    }

    pub fn print_board(&self) {
        println!("-------------");
        for i in self.ground.iter() {
            for j in i.iter() {
                print!("| {} ", j);
            }
            println!("|");
            println!("-------------");
        }
    }

    pub fn insert_state_at(&mut self, x: usize, y: usize, s: State) {
        self.last_played_state = s;
        self.ground[x][y] = s;
    }

    pub fn play(&mut self, x: usize, y: usize) {
        if self.last_played_state == State::O {
            self.insert_state_at(x, y, State::X);
        } else if self.last_played_state == State::X {
            self.insert_state_at(x, y, State::O);
        } else {
            self.insert_state_at(x, y, State::X);
        }
    }

    pub fn ground(&self) -> &Ground {
        &self.ground
    }

    pub fn last_played_state(&self) -> &State {
        &self.last_played_state
    }
}

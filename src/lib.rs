pub struct Field {
    current_state: Vec<u8>,
    next_state: Vec<u8>,
    width: usize,
    ruleset: u8,
    rules_function: fn(&mut Self) -> usize,
}

impl Field {
    pub fn new(width: usize, ruleset: u8, wrap_around: bool) -> Field {
        let rules_function: fn(&mut Self) -> usize = if wrap_around {
            Field::apply_rules_wrapped
        } else {
            Field::apply_rules_closed
        };
        Field {
            ruleset,
            width,
            rules_function,
            current_state: vec![0; width],
            next_state: vec![0; width],
        }
    }

    pub fn apply_rules(&mut self) -> usize {
        (self.rules_function)(self)
    }

    fn apply_rules_closed(&mut self) -> usize {
        let mut sum: usize = 0;
        for n in 1..self.width - 1 {
            sum += self.current_state[n] as usize;
            self.next_state[n] = (self.ruleset
                >> (self.current_state[(n + self.width - 1) % self.width] * 4
                    + self.current_state[n] * 2
                    + self.current_state[(n + 1) % self.width]))
                % 2;
        }
        sum
    }

    fn apply_rules_wrapped(&mut self) -> usize {
        let mut sum: usize = 0;
        for n in 0..self.width {
            sum += self.current_state[n] as usize;
            self.next_state[n] = (self.ruleset
                >> (self.current_state[(n + self.width - 1) % self.width] * 4
                    + self.current_state[n] * 2
                    + self.current_state[(n + 1) % self.width]))
                % 2;
        }
        sum
    }

    pub fn set_state(&mut self, i: usize, state: u8) {
        self.current_state[i % self.width] = state;
    }

    pub fn set_next_state(&mut self, i: usize, state: u8) {
        self.next_state[i % self.width] = state;
    }

    pub fn resize(&mut self, new_size: usize) {
        self.current_state.resize(new_size, 0);
        self.next_state.resize(new_size, 0);
        self.width = new_size;
    }

    pub fn swap_states(&mut self) {
        std::mem::swap(&mut self.current_state, &mut self.next_state);
    }

    pub fn print_states(&self) {
        for i in &self.current_state {
            if *i == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
}

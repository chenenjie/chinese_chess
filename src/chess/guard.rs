use chess::StepRule;
use board::get_map;
use chess::{Chess, Position, Group};

struct Guard;


impl StepRule for Guard {
    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool {
        let mut result_poins = Vec::new();
    }
}

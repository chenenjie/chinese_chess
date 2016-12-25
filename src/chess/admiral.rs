use chess::{StepRule, Group, Position, to_key};
use board::{get_map};


struct Admiral;

impl StepRule for Admiral {
    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool {
        
    }
}
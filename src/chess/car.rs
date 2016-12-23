use chess::{StepRule, Group, Position, to_key};
use board::{get_map};


struct Car;

impl StepRule for Car {
    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool {
        
    }
}
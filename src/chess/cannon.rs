
use chess::StepRule;


struct Cannon;

impl StepRule for Cannon {
    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool {
        
    }
}


pub const board_height: i32 = 10;
pub const board_width: i32 = 9;

pub enum Group {
    Black,
    Red,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn to_key(&self) -> String {
        format!("{}_{}", self.x, self.y)
    }
}

pub struct Chess<T : StepRule> {
    group: Group,
    position: Position,
    role: T,
}

impl<T :StepRule> Chess {
    fn new(group: Group, position: Position, role: T) -> Chess{
        Chess {
            group: group,
            position: position,
            role: role
        }
    }
}


pub trait StepRule {
     fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool;
}


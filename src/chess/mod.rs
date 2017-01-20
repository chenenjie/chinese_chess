pub mod admiral;
pub mod cannon;
pub mod car;
pub mod elephant;
pub mod guard;
pub mod horse;
pub mod soldier;

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
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y,
        }
    }
    fn to_key(&self) -> String {
        format!("{}_{}", self.x, self.y)
    }

    fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub struct Chess<T: StepRule> {
    group: Group,
    position: Position,
    role: T,
}

impl<T: StepRule> Chess<T> {
    pub fn new(group: Group, position: Position, role: T) -> Chess<T> {
        Chess {
            group: group,
            position: position,
            role: role,
        }
    }
}


pub trait StepRule {
     fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool;
}

pub trait ChessStyle{
    fn get_group(&self) -> &Group;
}

impl<T: StepRule> ChessStyle for Chess<T> {
    fn get_group(&self) -> &Group {
        self.group    
    }
}

use board::BoardMap;

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

    fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub struct Chess<T: StepRule> {
    group: Group,
    position: Position,
    role: Box<T>,
}

impl<T :StepRule> Chess<T> {
    fn new(group: Group, position: Position, role: T) -> Chess<T>{
        Chess {
            group: group,
            position: position,
            role: Box::new(role),
        }
    }
}


pub trait StepRule {
     fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool;
}

pub trait ChessStyle{
    fn get_self(&mut self) -> &mut Self;
}

pub fn to_key(x :i32, y: i32) -> String{
    format!("{}_{}", x, y);
}
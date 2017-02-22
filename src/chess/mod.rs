use std::collections::HashMap;

pub mod admiral;
pub mod cannon;
pub mod car;
pub mod elephant;
pub mod guard;
pub mod horse;
pub mod soldier;

pub use self::admiral::Admiral;
pub use self::cannon::Cannon;
pub use self::car::Car;
pub use self::elephant::Elephant;
pub use self::guard::Guard;
pub use self::horse::Horse;
pub use self::soldier::Soldier;
pub const board_height: i32 = 10;
pub const board_width: i32 = 9;

#[derive(PartialEq, Eq, Hash)]
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

pub struct Chess {
    pub group: Group,
    pub position: Position,
    pub role: Box<StepRule>,
}

impl Chess {
    pub fn new<T:StepRule>(group: Group, position: Position, role: T) -> Chess {
        Chess {
            group: group,
            position: position,
            role: Box::new(role) as Box<StepRule>,
        }
    }

    pub fn warp_get_next_step(&self, (x, y): (i32, i32), map: &HashMap<(i32, i32), Chess>) -> bool {
        self.role.get_next_step(&self.group, &self.position, &Position::new(x, y), map)
    }
}


pub trait StepRule:'static + Sync + Send{
     fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position, map: &HashMap<(i32, i32), Chess>) -> bool;
     fn get_type(&self) -> i32;
}

//pub trait ChessStyle{
    //fn get_group(&self) -> &Group;
//}

//impl<T: StepRule> ChessStyle for Chess<T> {
    //fn get_group(&self) -> &Group {
        //self.group    
    //}
//}

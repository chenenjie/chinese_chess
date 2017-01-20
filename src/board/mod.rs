use chess::ChessStyle;
use chess::{Chess, Group};
use chess::admiral::Admiral;
use chess::cannon::Cannon;
use chess::elephant::Elephant;
use chess::car::Car;
use chess::guard::Guard;
use chess::horse::Horse;
use chess::soldier::Soldier;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

lazy_static!{
    static ref BOARD_MAP: Arc<Mutex<HashMap<(i32, i32), Box<ChessStyle>>>> = Arc::new(Mutex::new(HashMap::new()));
}


pub fn get_map() -> Arc<Mutex<HashMap<(i32, i32), Box<ChessStyle>>>> {
    BOARD_MAP.clone();
}


struct BoardMap {
    map: HashMap<(i32, i32), Box<ChessStyle>>,
}


impl BoardMap {
    fn new() -> BoardMap {
        BoardMap {
            map: HashMap::new(),
        }
    }


    fn reset(&mut self) {
        self.map.clear();
    }

    fn init(&mut self) {
        self.map.insert((0, 0), Chess::new(Group::Red, Postion::new(0, 0), Car));
        self.map.insert((8, 0), Chess::new(Group::Red, Postion::new(0, 8), Car));
        self.map.insert((1, 0), Chess::new(Group::Red, Postion::new(0, 1), Horse));
        self.map.insert((7, 0), Chess::new(Group::Red, Postion::new(0, 7), Horse));
        self.map.insert((2, 0), Chess::new(Group::Red, Postion::new(0, 2), Elephant));
        self.map.insert((6, 0), Chess::new(Group::Red, Postion::new(0, 6), Elephant));
        self.map.insert((3, 0), Chess::new(Group::Red, Postion::new(0, 3), Guard));
        self.map.insert((5, 0), Chess::new(Group::Red, Postion::new(0, 5), Guard));
        self.map.insert((4, 0), Chess::new(Group::Red, Postion::new(0, 4), Admiral));

        self.map.insert((1, 2), Chess::new(Group::Red, Postion::new(1, 2), Cannon));
        self.map.insert((7, 2), Chess::new(Group::Red, Postion::new(7, 2), Cannon));
        self.map.insert((0, 3), Chess::new(Group::Red, Postion::new(0, 3), Soldier));
        self.map.insert((2, 3), Chess::new(Group::Red, Postion::new(2, 3), Soldier));
        self.map.insert((4, 3), Chess::new(Group::Red, Postion::new(4, 3), Soldier));
        self.map.insert((6, 3), Chess::new(Group::Red, Postion::new(6, 3), Soldier));
        self.map.insert((8, 3), Chess::new(Group::Red, Postion::new(8, 3), Soldier));

    }
}
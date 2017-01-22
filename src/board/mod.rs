use chess::ChessStyle;
use chess::{Chess, Group, Position};
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
        self.map.insert((0, 0), Chess::new(Group::Red, Position::new(0, 0), Car));
        self.map.insert((8, 0), Chess::new(Group::Red, Position::new(8, 0), Car));
        self.map.insert((1, 0), Chess::new(Group::Red, Position::new(1, 0), Horse));
        self.map.insert((7, 0), Chess::new(Group::Red, Position::new(7, 0), Horse));
        self.map.insert((2, 0), Chess::new(Group::Red, Position::new(2, 0), Elephant));
        self.map.insert((6, 0), Chess::new(Group::Red, Position::new(6, 0), Elephant));
        self.map.insert((3, 0), Chess::new(Group::Red, Position::new(3, 0), Guard));
        self.map.insert((5, 0), Chess::new(Group::Red, Position::new(5, 0), Guard));
        self.map.insert((4, 0), Chess::new(Group::Red, Position::new(4, 0), Admiral));

        self.map.insert((1, 2), Chess::new(Group::Red, Position::new(1, 2), Cannon));
        self.map.insert((7, 2), Chess::new(Group::Red, Position::new(7, 2), Cannon));
        self.map.insert((0, 3), Chess::new(Group::Red, Position::new(0, 3), Soldier));
        self.map.insert((2, 3), Chess::new(Group::Red, Position::new(2, 3), Soldier));
        self.map.insert((4, 3), Chess::new(Group::Red, Position::new(4, 3), Soldier));
        self.map.insert((6, 3), Chess::new(Group::Red, Position::new(6, 3), Soldier));
        self.map.insert((8, 3), Chess::new(Group::Red, Position::new(8, 3), Soldier));

        self.map.insert((0, 9), Chess::new(Group::Black, Position::new(0, 9), Car));
        self.map.insert((8, 9), Chess::new(Group::Black, Position::new(8, 9), Car));
        self.map.insert((1, 9), Chess::new(Group::Black, Position::new(1, 9), Horse));
        self.map.insert((7, 9), Chess::new(Group::Black, Position::new(7, 9), Horse));
        self.map.insert((2, 9), Chess::new(Group::Black, Position::new(2, 9), Elephant));
        self.map.insert((6, 9), Chess::new(Group::Black, Position::new(6, 9), Elephant));
        self.map.insert((3, 9), Chess::new(Group::Black, Position::new(3, 9), Guard));
        self.map.insert((5, 9), Chess::new(Group::Black, Position::new(5, 9), Guard));
        self.map.insert((4, 9), Chess::new(Group::Black, Position::new(4, 9), Admiral));

        self.map.insert((1, 7), Chess::new(Group::Black, Position::new(1, 7), Cannon));
        self.map.insert((7, 7), Chess::new(Group::Black, Position::new(7, 7), Cannon));
        self.map.insert((0, 6), Chess::new(Group::Black, Position::new(0, 6), Soldier));
        self.map.insert((2, 6), Chess::new(Group::Black, Position::new(2, 6), Soldier));
        self.map.insert((4, 6), Chess::new(Group::Black, Position::new(4, 6), Soldier));
        self.map.insert((6, 6), Chess::new(Group::Black, Position::new(6, 6), Soldier));
        self.map.insert((8, 6), Chess::new(Group::Black, Position::new(8, 6), Soldier));

    }
}

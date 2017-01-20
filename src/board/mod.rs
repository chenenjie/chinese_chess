use std::collections::HashMap;
use std::sync::{Mutex, Arc};

lazy_static!{
    static ref BOARD_MAP: Arc<Mutex<HashMap<(i32, i32), Box<ChessStyle>>>> = Arc::new(Mutex::new(HashMap::new()));
}


pub fn get_map() -> Arc<Mutex<HashMap<(i32, i32), Box<ChessStyle>>> {
    BOARD_MAP.clone();
}
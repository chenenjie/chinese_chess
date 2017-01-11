use std::collections::HashMap;
use chess::{Chess, StepRule};
use std::sync::{Arc, Mutex};


lazy_static!{
    pub ref BOARD_MAP: Arc<Mutx<HashMap<String, ChessStyle>>> = Arc::new(Mutex::new(HashMap::new));
}

pub fn get_map() -> Arc<Mutex<HashMap<String, Box<ChessStyle>>>> {
    BOARD_MAP.clone()
}

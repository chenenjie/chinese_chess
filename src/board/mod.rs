use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use chess::Chess;



lazy_static! {
    pub static ref BOARD_CHESS_MAP :Arc<Mutex<HashMap<String, Chess>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn get_map() -> Arc<Mutex<HashMap<String, Chess>>> {
    BOARD_CHESS_MAP.clone()
}

pub fn init_map() {
    {
        let data = BOARD_CHESS_MAP.lock().unwrap();
        
    }
}

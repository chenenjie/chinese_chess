use std::collections::HashMap;

use chess::{Chess, StepRule};



pub struct BoardMap{
    pub map : HashMap<String, Chess>,
}

impl BoardMap {
    fn new() -> BoardMap{
        BoardMap {
            map: HashMap::new(),    
        }
    }
}

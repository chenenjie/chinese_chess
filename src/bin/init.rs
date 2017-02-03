extern crate chinese_chess;
use chinese_chess::board::{get_map,init};

fn main(){
    init();
    
    {
        let arc_map = get_map();
        let map = arc_map.lock().unwrap();

        println!("{}",map.get(&(0, 0)).unwrap().warp_get_next_step((1, 1), &( *map) ));
        
    }
}

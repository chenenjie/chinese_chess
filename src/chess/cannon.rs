use board::get_map;
use chess::{StepRule, get_map, to_key};


struct Cannon;

impl StepRule for Cannon {
     fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position, &mut board_map: BoardMap) -> bool;
        let result_points = Vec::new();

        match my_position {
            Position{x: x, y: y} => {
                let map = get_map().lock().unwrap();
                
                let mut index = 0;
                for i in x-1..0 {
                    if index == 0 {
                        if let Some(chess) = map.get(to_key(i, y)){
                            index = index + 1;
                        }else{
                            result_points.push((i, y));
                        }
                    } else{
                        if let Some(chess) = map.get(to_key(i, y)){
                            if side != chess.group {
                                result_points.push((i,y))
                            }
                        }
                    }
                }

                index = 0;
                for i in x+1..8 {
                    if index == 0 {
                        if let Some(chess) = map.get(to_key(i, y)){
                            index = index + 1;
                        }else{
                            result_points.push((i, y));
                        }
                    } else{
                        if let Some(chess) = map.get(to_key(i, y)){
                            if side != chess.group {
                                result_points.push((i,y));
                            }
                        }
                    }
                }
                
                index = 0;
                for i in y-1..0 {
                    if index == 0 {
                        if let Some(chess) = map.get(to_key(x, i)){
                            index = index + 1;
                        }else {
                            result_points.push((x, i));
                        }
                    }else {
                        if let Some(chess) = map.get(to_key(x, i)){
                            if side != chess.group {
                                result_points.push((x, i));
                            }
                        }
                    }
                }

                index = 0;
                for i in y+1..9 {
                    if index == 0 {
                        if let Some(chess) = map.get(to_key(x, i)){
                            index = index + 1;
                        }else {
                            result_points.push((x, i));
                        }
                    }else {
                        if let Some(chess) = map.get(to_key(x, i)){
                            if side != chess.group {
                                result_points.push((x, i));
                            }
                        }
                    }
                }

                if result_points.contains(position) {
                    true
                }else{
                    false
                }
            }
        }
        
    }
}


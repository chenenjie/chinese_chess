use chess::{StepRule,Chess, Group, Position};
use std::collections::HashMap;
use board::get_map;

pub struct Cannon;

impl StepRule for Cannon {
    fn get_type(&self) -> i32 {
        return 4;
    }
     fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position, map: &HashMap<(i32, i32), Chess>) -> bool{
        let mut result_points = Vec::new();

        match *my_position {
            Position{ref x, ref y} => {
                //let arc_map = get_map();
                //let map = arc_map.lock().unwrap();
                
                let mut index = 0;
                for i in 0..x-1 {
                    //x=0的时候不用检查
                    let temp = x-1-i;

                    if index == 0 {
                        if let Some(chess) = map.get(&(temp, *y)){
                            index = index + 1;
                        }else{
                            result_points.push((temp, *y));
                        }
                    } else{
                        if let Some(chess) = map.get(&(temp, *y)){
                            if *side != chess.group {
                                result_points.push((temp,*y))
                            }
                        }
                    }
                }

                index = 0;
                for i in x+1..9 {
                    if index == 0 {
                        if let Some(chess) = map.get(&(i, *y)){
                            index = index + 1;
                        }else{
                            result_points.push((i, *y));
                        }
                    } else{
                        if let Some(chess) = map.get(&(i, *y)){
                            if *side != chess.group {
                                result_points.push((i, *y));
                            }
                        }
                    }
                }
                
                index = 0;
                for i in 0..y-1{
                    let temp = y-1-i;
                    if index == 0 {
                        if let Some(chess) = map.get(&(*x, temp)){
                            index = index + 1;
                        }else {
                            result_points.push((*x, temp));
                        }
                    }else {
                        if let Some(chess) = map.get(&(*x, temp)){
                            if *side != chess.group {
                                result_points.push((*x, temp));
                            }
                        }
                    }
                }

                index = 0;
                for i in y+1..10 {
                    if index == 0 {
                        if let Some(chess) = map.get(&(*x, i)){
                            index = index + 1;
                        }else {
                            result_points.push((*x, i));
                        }
                    }else {
                        if let Some(chess) = map.get(&(*x, i)){
                            if *side != chess.group {
                                result_points.push((*x, i));
                            }
                        }
                    }
                }

                if result_points.contains(&(position.to_tuple())) {
                    true
                }else{
                    false
                }
            }
        }
        
    }
}


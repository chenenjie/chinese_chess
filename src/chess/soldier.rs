use chess::{StepRule, Group, Position, Chess};
use std::collections::HashMap;
use board::get_map;

pub struct Soldier;

impl StepRule for Soldier {
    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position, map: &HashMap<(i32, i32), Chess>) -> bool{
        let mut result_points = Vec::new();

        match *my_position {
            Position{ref x, ref y} => {
                if *side == Group::Red {
                    result_points.push((*x, y+1));
                    if *y > 4 {
                        result_points.push((x-1, *y));
                        result_points.push((x+1, *y));
                    }
                }else{
                    result_points.push((*x, y-1));
                    if *y < 5 {
                        result_points.push((x-1, *y));
                        result_points.push((x+1, *y));
                    }
                }

                //let arc_map = get_map();
                //let map = arc_map.lock().unwrap();
                
                //判断是否己方棋子
                result_points.iter().filter(|&&(x, y)| {
                    if let Some(chess) = map.get(&(x, y)){
                        if chess.group == *side {
                            return false;
                        }
                    }
                    true
                });
            }
        }



        if result_points.contains(&(position.to_tuple())) {
            true
        }else {
            false
        }
    }
}

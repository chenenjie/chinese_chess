use std::collections::HashMap;
use chess::{StepRule, Group, Position, Chess};
use board::{get_map};


pub struct Admiral;

impl StepRule for Admiral {

    fn get_type(&self) -> i32 {
        return 0;
    }

    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position, map: &HashMap<(i32, i32), Chess>) -> bool{
        let mut result_points = Vec::new();
        
        {
            match *my_position {
                Position {ref x, ref y} => {

                    //一步可走的距离
                    result_points.push((x+1, y+1));
                    result_points.push((x+1, y-1));
                    result_points.push((x-1, y+1));
                    result_points.push((x-1, y-1));

                    //筛选是否走出田字框
                    result_points.iter().filter(|&&(x, y)| {
                        if *side == Group::Red {
                            x >= 3 && x <= 5 && y >= 0 && y <= 2
                        }else {
                            x >= 3 && x <= 5 && y >= 7 && y <= 9
                        }
                    });

                    //判断目标地点是否有己方
                    //let arc_map = get_map();
                    //let map = arc_map.lock().unwrap();
                    result_points.iter().filter(|&&(x, y)| {
                        if let Some(chess) = map.get(&(x, y)){
                            if *side == chess.group {
                                false
                            } else {
                                true
                            }
                        } else {
                            true
                        }
                    });

                    if result_points.contains(&(position.to_tuple())) {
                        true
                    }else {
                        false
                    }
                }
            }
        }
    }
}

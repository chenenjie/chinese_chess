use chess::{Chess,StepRule, Group, Position};
use std::collections::HashMap;
use board::{get_map};


pub struct Car;

impl StepRule for Car {
    fn get_type(&self) -> i32 {
        return 5;
    }
    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position, map: &HashMap<(i32, i32), Chess>) -> bool{
        let mut result_points = Vec::new();
    
        //判断是否有直线上是否有棋子
        {
            //let arc_map = get_map();
            //let map = arc_map.lock().unwrap();
            match *my_position {
                Position{x:ref x, y: ref y} => {
                    //从中间到四周进行判断
                    for i in 0..x-1 {
                        let temp = x-1-i;
                        if let Some(chess) = map.get(&(temp, *y)) {
                            if chess.group != *side {
                                result_points.push((temp, *y));
                            }

                            break;
                        }
                        result_points.push((temp, *y));
                    }
                    for i in x+1..9 {
                        if let Some(chess) = map.get(&(i, *y)){
                            if chess.group != *side {
                                result_points.push((i, *y));
                            }

                            break;
                        }
                        result_points.push((i, *y));
                    }

                    for i in 0..y-1{
                        let temp = y-1-i;
                        if let Some(chess) = map.get(&(*x, temp)){
                            if chess.group != *side {
                                result_points.push((*x, temp));
                            }
                            break;
                        }
                        result_points.push((*x, temp));
                    }

                    for i in y+1..10 {
                        if let Some(chess) = map.get(&(*x, i)){
                            if chess.group != *side {
                                result_points.push((*x, i));
                            }
                            break;
                        }
                        result_points.push((*x, i));
                    }
                }
            }
        }

        if result_points.contains(&(position.to_tuple())) {
            true
        } else {
            false
        }
    }
}

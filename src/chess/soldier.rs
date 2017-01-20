use chess::{StepRule, Group, to_key};
use board::get_map;

struct Soldier;

impl StepRule for Soldier {
     fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool;
        let result_points = Vec::new();

        match my_position {
            Position{x: x, y: y} => {
                if side == Group::Red {
                    result_points.push((x, y+1));
                    if y > 4 {
                        result_points.push((x-1, y));
                        result_points.push((x+1, y));
                    }
                }else{
                    result_points.push((x, y-1));
                    if y < 5 {
                        result_points.push((x-1, y));
                        result_points.push((x+1, y));
                    }
                }

                let map = get_map().lock().unwrap();
                
                //判断是否己方棋子
                result_points.iter().filter(|(x, y)| {
                    if let Some(chess) = map.get(to_key(x, y)){
                        if chess.group == side {
                            false
                        }

                        true
                    }
                });
            }
        }



        if result_points.contains(position) {
            true
        }else {
            false
        }
    }
}
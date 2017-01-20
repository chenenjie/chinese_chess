use chess::{StepRule, Group, Position};
use board::{get_map};

pub struct Elephant;

impl StepRule for Elephant {
    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool;
        let result_point = Vec::new();
        let map = get_map().lock().unwrap();

        match  my_position {
            Position{ref x, ref y} => {
                //判断塞象眼
                if let None = map.get((x+1, y+1)) {
                    result_point.push(x+2, y+2);
                }
                if let None = map.get((x-1, y+1)){
                    result_point.push(x-2, y+2);
                }
                if let None = map.get((x-1, y-1)){
                    result_point.push(x-2, y-2);
                }
                if let None = map.get((x+1, y-1)){
                    result_point.push(x+2, y-2);
                }
                //判断是否超出界限
                if side == Group::Red {
                    result_point.iter().filter(|(x, y)|{
                        if x >= 0 && x <= 8 && y >= 0 && y <= 4 {
                            true
                        } else{
                            false
                        }
                    });
                } else {
                    result_point.iter().filter(|(x, y)|{
                        if x >= 0 && x <= 8 && y >= 5 && y <= 9 {
                            true
                        } else{
                            false
                        }
                    });
                }
                
                //判断是否有友方的棋子在
                result_point.iter().filter(|(x, y)| {
                    if let Some(chess) = map.get((x, y)){
                        if side == chess.group {
                            false
                        }else {
                            true
                        }
                    }
                });     

                if result_point.contains(position.to_tuple) {
                    return true;
                }
                false
                    
            }
        }
    }
}

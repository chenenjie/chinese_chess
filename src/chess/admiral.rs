use chess::{StepRule, Group, Position, to_key};
use board::{get_map};


struct Admiral;

impl StepRule for Admiral {
    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool {
        let result_points = Vec::new();
        
        {
            matchd my_position {
                Position {x:x, y:y} => {

                    //一步可走的距离
                    result_points.push(x+1, y+1);
                    result_points.push(x+1, y-1);
                    result_points.push(x-1, y+1);
                    result_points.push(x-1, y-1);

                    //筛选是否走出田字框
                    result_points.iter().filter(|(x, y)| {
                        if side == Group::Red {
                            x >= 3 && x <= 5 && y >= 0 && y <= 2
                        }else {
                            x >= 3 && x <= 5 && y >= 7 && y <= 9
                        }
                    });

                    //判断目标地点是否有己方
                    let map = get_map.lock().unwrap();
                    result_points.iter().filter(|(x, y)| {
                        if let Some(chess) = map.get(to_key(x, y)){
                            if side == chess.group {
                                false
                            } else {
                                true
                            }
                        } else {
                            true
                        }
                    });

                    if result_points.contains(position) {
                        true
                    }else {
                        false
                    }

                    
                }
            }
        }
    }
}
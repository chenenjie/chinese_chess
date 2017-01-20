use chess::StepRule;
use board::get_map;
use chess::{Chess, Position, Group, to_key};

struct Guard;


impl StepRule for Guard {
     fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool;
        let mut result_points = Vec::new();
        
        result_points.push(3, 0);
        result_points.push(5, 0);
        result_points.push(4, 1);
        result_points.push(3, 2);
        result_points.push(5, 2);

        result_points.push(3, 9);
        result_points.push(5, 9);
        result_points.push(4, 8);
        result_points.push(3, 7);
        result_points.push(5, 7);

        //根据红方或者是黑方筛选点
        result_points.iter().filter(|(x, y)|{
            if side == Group::Red {
                if y < 4 {
                    true
                }else{
                    false
                }
            }else{
                if y > 5 {
                    true
                }else{
                    false
                }
            }
        });
        
        //判断目标点和当前位置是否有一步的距离
        result_points.iter().filter(|(x, y)|{
            x - my_position.x == 1 || x - my_position == -1) && (y - my_position.y == 1 || y - my_position.y == -1) 
        });

        //判断可选目标点是否有
        {

            let map = get_map().lock().unwrap();

            result_points.iter().filter(|(x, y)|{
                if let Some(chess) = map.get(to_key(x, y)) {
                    if side == chess.group {
                        return false
                    }
                }

                true
            });
        }

        if result_points.contains(position) {
            true
        }else {
            false
        } 
    }
}

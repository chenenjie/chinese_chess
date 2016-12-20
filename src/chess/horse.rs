use chess::StepRule;
use board::get_map;
use chess::{Chess, Position, Group};

#[derive(Debug)]
struct Horse;



impl StepRule for Horse {
    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool{
        //获取周围的点
        let mut result_points = Vec::new();
        let (x, y) = my_position;
        {
            //绊马脚判断
            let map = get_map().lock().unwrap();
            if let Some(chess) = map.get(to_key(x+1, y) {
                let x = chess.position.x;
                let y = chess.position.y;
                result_points.push(x+2, y+1);
                result_points.push(x+2, y-1);
            }
            if let Some(chess) = map.get(to_key(x-1, y) {
                let x = chess.position.x;
                let y = chess.position.y;
                result_points.push(x-2, y+1);
                result_points.push(x-2, y-1);
            }
            if let Some(chess) = map.get(to_key(x, y+1) {
                let x = chess.position.x;
                let y = chess.position.y;
                result_points.push(x-1, y+2);
                result_points.push(x+1, y+2);
            }
            if let Some(chess) = map.get(to_key(x, y-1) {
                let x = chess.position.x;
                let y = chess.position.y;
                result_points.push(x+1, y-2);
                result_points.push(x-1, y-2);
            }
            
            //判断目标地点是否有友方棋子
            result_points.iter().filter(|(x, y)| {
                if let Some(chess) = map.get(to_key(x, y)){
                    if(chess.group == side){
                        return false;
                    }
                }
                true
            });
            
        }
        
        //判断是否在棋盘内
        result_points.iter().filter(|(x, y)| x >= 0 && x<9 && y>=0 && y<10)
        
        if result_points.contains(position) {
            true
        }else {
            false
        }
    }
}

fn to_key(x :i32, y: i32) -> String{
    format!("{}_{}", x, y);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_fe() {
        assert_eq!(4, 4);
    }
}





use chess::{StepRule, Group, Position};
use board::{get_map};


pub struct Car;

impl StepRule for Car {
    fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool;
        let result_points = Vec::new();
        
    
        //判断是否有直线上是否有棋子
        {
            let map = get_map().lock().unwrap();
            match my_position {
                Position{x:ref x, y: ref y} => {
                    //从中间到四周进行判断
                    if x != 0  {
                        for i in x-1..0 {
                            if let Some(chess) = map.get((i, y)) {
                                if chess.group != side {
                                    result_points.push(i, y);
                                }

                                break;
                            }
                            result_points.push(i, y);
                        }
                    }
                    if x != 8 {
                        for i in x+1..8 {
                            if let Some(chess) = map.get((i, y)){
                                if chess.group != side {
                                    result_points.push(i, y);
                                }

                                break;
                            }
                            result_points.push(i, y);
                        }
                    }

                    if y != 0 {
                        for i in y-1..0{
                            if let Some(chess) = map.get((x, i)){
                                if chess.group != side {
                                    result_points.push(x, i);
                                }
                                break;
                            }
                            result_points.push(x, i);
                        }
                    }

                    if y != 9 {
                        for i in y+1..9 {
                            if let Some(chess) = map.get((x, i)){
                                if chess.group != side {
                                    result_points.push(x, i);
                                }
                                break;
                            }
                            result_points.push(x, i);
                        }
                    }
                }
            }
        }

        if result_points.contains(position) {
            true
        } else {
            false
        }
    }
}

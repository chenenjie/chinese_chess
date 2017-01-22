use chess::{StepRule, Group, Position};

pub struct Cannon;

impl StepRule for Cannon {
     fn get_next_step(&self, side: &Group, my_position: &Position, position: &Position) -> bool{
        let result_points = Vec::new();

        match my_position {
            Position{ref x, ref y} => {
                let map = get_map().lock().unwrap();
                
                let mut index = 0;
                for i in x-1..0 {
                    if index == 0 {
                        if let Some(chess) = map.get((i, y)){
                            index = index + 1;
                        }else{
                            result_points.push((i, y));
                        }
                    } else{
                        if let Some(chess) = map.get((i, y)){
                            if side != chess.group {
                                result_points.push((i,y))
                            }
                        }
                    }
                }

                index = 0;
                for i in x+1..8 {
                    if index == 0 {
                        if let Some(chess) = map.get((i, y)){
                            index = index + 1;
                        }else{
                            result_points.push((i, y));
                        }
                    } else{
                        if let Some(chess) = map.get((i, y)){
                            if side != chess.group {
                                result_points.push((i,y));
                            }
                        }
                    }
                }
                
                index = 0;
                for i in y-1..0 {
                    if index == 0 {
                        if let Some(chess) = map.get((x, i)){
                            index = index + 1;
                        }else {
                            result_points.push((x, i));
                        }
                    }else {
                        if let Some(chess) = map.get((x, i)){
                            if side != chess.group {
                                result_points.push((x, i));
                            }
                        }
                    }
                }

                index = 0;
                for i in y+1..9 {
                    if index == 0 {
                        if let Some(chess) = map.get((x, i)){
                            index = index + 1;
                        }else {
                            result_points.push((x, i));
                        }
                    }else {
                        if let Some(chess) = map.get((x, i)){
                            if side != chess.group {
                                result_points.push((x, i));
                            }
                        }
                    }
                }

                if result_points.contains(position) {
                    true
                }else{
                    false
                }
            }
        }
        
    }
}


extern crate piston_window;
extern crate find_folder;
extern crate opengl_graphics;
extern crate image as oimage;
extern crate chinese_chess;

use piston_window::{PistonWindow, OpenGL, WindowSettings, Flip, Input, Context, clear, color, image, EventLoop, polygon, Transformed};
use piston_window::{MouseButton, Button, line, rectangle};
use piston_window::{MouseCursorEvent, MouseRelativeEvent};
use opengl_graphics::{GlGraphics,TextureSettings};
use opengl_graphics::Texture;
use piston_window::rectangle::square;
use oimage::{DynamicImage, RgbaImage, GenericImage};
use std::path::Path;
use chinese_chess::board::{get_map, init};
use chinese_chess::chess::{Position, Admiral, Car, Elephant, Cannon, Guard, Horse, Soldier, StepRule, Group};

const CHESS_BOUND :f64 = 120f64;
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const VIOLET: [f32; 4] = [0.6, 0.0, 1.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston: image", [560, 620])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::Parents(3)
        .for_folder("assets").unwrap();
    let board_background = assets.join("timg.jpg");

    let mut origin_image = get_oringin_image(assets.join("chess.jpg")).unwrap();

    let board_background = Texture::from_path(
            &board_background,
        ).unwrap();
    
    
    
    let mut app = App::new(board_background, &mut origin_image);
    let mut gl = GlGraphics::new(opengl);
    window.set_lazy(true);


    while let Some(e) = window.next(){
        app.refresh_point(e.mouse_cursor_args());
        // app.refresh_point(e.mouse_relative_args());
        match e {
            Input::Press(Button::Mouse(button)) => {
                app.press_button(button);
            },
            Input::Release(Button::Mouse(button)) => {
                app.release_button(button);
            },
            Input::Render(args) => {
                gl.draw(args.viewport(),|c, g| app.render(c, g));
            }, 
            _ => {}
        }
    }
}

struct App{
    background : Texture,
    image: Vec<Texture>,
    left_press : bool,
    right_press : bool,
    position : Option<(f64, f64)>,
    duration : i32, //0:xuanqi 1:xuanbu
    // former_press : bool,
    chosen_chess : Option<(i32, i32)>,
    turn : Group,
}


impl App {
    fn change_side(&mut self) {
        if self.turn == Group::Red {
            self.turn = Group::Black;
        } else{
            self.turn = Group::Red;
        }
    }
    fn render(&mut self, c: Context, g: &mut GlGraphics){
        clear(color::BLACK, g);
        image(&self.background, c.transform, g);
        // image.draw(&background, default_draw_state(), c.transform, g);

        // for i in 0..self.image.len() {
        //     image(&self.image[i], c.transform.trans(x, y).zoom(0.45), g);
        //     x += (CHESS_BOUND - 35f64);
        // }
        // image(&self.image[0], c.transform.trans(x, y).zoom(0.45), g);
        // image(&self.image[0], c.transform.trans(App::to_x(5), App::to_y(4)).zoom(0.45), g);
        {
            let arc_map = get_map();        
            let mut map = arc_map.lock().unwrap();
            for (key, value) in map.iter(){
                let &(x, y) = key;
                match value.role.get_type() {
                    i @ 0...7 =>{
                        if value.group == Group::Black {
                            image(&self.image[i as usize], c.transform.trans(App::to_x(x), App::to_y(y)).zoom(0.45), g);
                        } else {
                            image(&self.image[(i + 7) as usize], c.transform.trans(App::to_x(x), App::to_y(y)).zoom(0.45), g);
                        }
                    },
                    _ => {},
                }
                
            }
            //draw turn 
            if self.turn == Group::Red {
                rectangle(RED, [0f64, 0f64, 20f64, 20f64], c.transform.trans(270f64, 300f64), g);
            } else {
                rectangle(color::BLACK, [0f64, 0f64, 20f64, 20f64], c.transform.trans(270f64, 300f64), g);
            }

            //选定阶段
            if self.duration == 0 {
                if let Some(pos) = self.position {
                    let (x, y) = self.trans_position(pos);
                    let left_top :(f64, f64) = (App::to_x(x), App::to_y(y));
                    let left_bottom :(f64, f64) = (left_top.0, left_top.1 + 54f64);
                    let right_top :(f64, f64) = (left_top.0 + 54f64, left_top.1);
                    let right_bottom :(f64, f64) = (left_top.0 + 54f64, left_top.1 + 54f64);

                    if self.left_press == false{
                        line(RED, 1f64, [0f64, 0f64, 10f64, 0f64], c.transform.trans(left_top.0, left_top.1), g);
                        line(RED, 1f64, [0f64, 0f64, 0f64, 10f64], c.transform.trans(left_top.0, left_top.1), g);
                        
                        line(RED, 1f64, [0f64, 0f64, 10f64, 0f64], c.transform.trans(left_bottom.0, left_bottom.1), g);
                        line(RED, 1f64, [0f64, 0f64, 0f64, -10f64], c.transform.trans(left_bottom.0, left_bottom.1), g);

                        line(RED, 1f64, [0f64, 0f64, -10f64, 0f64], c.transform.trans(right_top.0, right_top.1), g);
                        line(RED, 1f64, [0f64, 0f64, 0f64, 10f64], c.transform.trans(right_top.0, right_top.1), g);

                        line(RED, 1f64, [0f64, 0f64, -10f64, 0f64], c.transform.trans(right_bottom.0, right_bottom.1), g);
                        line(RED, 1f64, [0f64, 0f64, 0f64, -10f64], c.transform.trans(right_bottom.0, right_bottom.1), g);
                    }else {
                        println!("click postion : {:?}", pos);
                        if let Some(chess) = map.get(&(x, y)) {
                            if chess.group == self.turn {
                                self.duration = 1;
                                self.chosen_chess = Some((x, y));
                                polygon(RED, &[[0f64, 0f64], [10f64, 0f64], [0f64, 10f64]], c.transform.trans(left_top.0, left_top.1), g); 
                                polygon(RED, &[[0f64, 0f64], [10f64, 0f64], [0f64, -10f64]], c.transform.trans(left_bottom.0, left_bottom.1), g);
                                polygon(RED, &[[0f64, 0f64], [-10f64, 0f64], [0f64, 10f64]], c.transform.trans(right_top.0, right_top.1), g);
                                polygon(RED, &[[0f64, 0f64], [-10f64, 0f64], [0f64, -10f64]], c.transform.trans(right_bottom.0, right_bottom.1), g);
                            }
                        }
                    }
                } 
            } else {
                if let Some((x, y)) = self.chosen_chess {
                    let left_top :(f64, f64) = (App::to_x(x), App::to_y(y));
                    let left_bottom :(f64, f64) = (left_top.0, left_top.1 + 54f64);
                    let right_top :(f64, f64) = (left_top.0 + 54f64, left_top.1);
                    let right_bottom :(f64, f64) = (left_top.0 + 54f64, left_top.1 + 54f64);

                    polygon(RED, &[[0f64, 0f64], [10f64, 0f64], [0f64, 10f64]], c.transform.trans(left_top.0, left_top.1), g); 
                    polygon(RED, &[[0f64, 0f64], [10f64, 0f64], [0f64, -10f64]], c.transform.trans(left_bottom.0, left_bottom.1), g);
                    polygon(RED, &[[0f64, 0f64], [-10f64, 0f64], [0f64, 10f64]], c.transform.trans(right_top.0, right_top.1), g);
                    polygon(RED, &[[0f64, 0f64], [-10f64, 0f64], [0f64, -10f64]], c.transform.trans(right_bottom.0, right_bottom.1), g);
                }
                
                if self.right_press == true {
                    self.duration = 0;
                    self.chosen_chess = None;
                } else {
                    if self.left_press == true {
                        if let Some(pos) = self.position {
                            if let Some((x, y)) = self.chosen_chess {
                                let po = self.trans_position((pos.0, pos.1));
                                let mut is_modify = false;
                                if let Some(chess) = map.get(&(x, y)) {
                                    let is_avaiable = chess.role.get_next_step(&chess.group, &chess.position, &Position::new(po.0 as i32, po.1 as i32), &*map);
                                    if is_avaiable == true {
                                        is_modify = true;
                                    }
                                }
                                if is_modify == true {
                                    if let Some(mut chess) = map.remove(&(x, y)) {
                                        chess.position = Position::new(po.0 as i32, po.1 as i32);
                                        map.insert((po.0, po.1), chess);
                                    };
                                    self.duration = 0;
                                    self.chosen_chess = None;
                                    self.change_side();
                                }
                            }
                        } 
                    }
                }
            }
        }


        //when left press print the postion
        // println!("{:?}", self.left_press);
        // println!("position : {:?}", self.position);
        
        // if self.left_press == true {
        //     if let Some(pos) = self.position{
        //         println!("position : {:?}", pos);
        //         println!("after translate :{:?}", self.trans_position(pos));
        //     }
        // }
    }

    fn new(background: Texture, origin_image: &mut RgbaImage) -> App{
        init();
        App{
            background: background,
            image: App::splite_single_chess(origin_image),
            left_press: false,
            right_press: false,
            position: None,
            duration: 0i32,
            // former_press: false,
            chosen_chess: None,
            turn : Group::Red,
        }        
    }

    fn splite_single_chess(origin_image: &mut RgbaImage) -> Vec<Texture>{
        let mut vec = vec![];
        // let (x, y) = sub_image.dimensions();
        // let (a,b,c,d) = sub_image.bounds();

        // println!("x: {}, y: {}", x, y);
        // println!("a: {}, b: {}, c: {}, d: {} ", a, b, c, d);
        let mut origin_x = 29u32;
        let mut origin_y = 61u32;
        for i in 1..8 {
            let mut sub_image = origin_image.sub_image(origin_x, origin_y, CHESS_BOUND as u32, CHESS_BOUND as u32);
            vec.push(Texture::from_image(&sub_image.to_image(), &TextureSettings::new()));
            origin_x += (CHESS_BOUND as u32);
        }
        origin_x = 29u32;
        origin_y += (CHESS_BOUND as u32) * 2;
        for i in 1..8 {
            let mut sub_image = origin_image.sub_image(origin_x, origin_y, CHESS_BOUND as u32, CHESS_BOUND as u32);
            vec.push(Texture::from_image(&sub_image.to_image(), &TextureSettings::new()));
            origin_x += (CHESS_BOUND as u32);
        }
        vec
    }

    fn press_button(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                // self.former_press = self.left_press;
                self.left_press = true;
            },
            MouseButton::Right => {
                self.right_press = true;
            },
            _ => {},
        }
    }

    fn release_button(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.left_press = false;
            },
            MouseButton::Right => {
                self.right_press = false;
            },
            _ => {},
        }
    }

    fn refresh_point(&mut self, point: Option<[f64; 2]>) {
        // println!("{:?}", point);
        if let Some(pos) = point {
            self.position = Some((pos[0], pos[1]));
        }
    }

    fn to_x(x :i32) -> f64 {
        (60 * x + 40 - 27) as f64
    }

    fn to_y(y :i32) -> f64 {
        (582 - (60 * y) -27) as f64
    }

    fn trans_position(&self, (x, y): (f64, f64)) -> (i32, i32) {
        let mut n = (x -40f64) / 60f64;
        let mut m = (582f64 - y) / 60f64;
        // println!("{:?} , {:?}", n, m);
        // println!("{:?} , {:?}", (n - n.floor()), (n.floor() + 1f64 - n));
        if n - n.floor() >= n.floor() + 1f64 - n{
            n = n.floor() + 1f64;
        } else {
            n = n.floor();
        }

        if m - m.floor() >= m.floor() + 1f64 - m{
            m = m.floor() + 1f64;
        } else {
            m = m.floor();
        }
        (n as i32, m as i32)
    } 
}


fn get_oringin_image<P>(path : P) -> Result<RgbaImage, String> where P: AsRef<Path> {
    let path = path.as_ref();

    let img = match oimage::open(path) {
        Ok(img) => img,
        Err(e) => {
            return Err(format!("Could not load the path '{:?}' : '{:?}'", path.file_name().unwrap(), e))
        }
    };

    let img = match img {
        DynamicImage::ImageRgba8(img) => img,
        x => x.to_rgba(),
    };

    Ok(img)
}

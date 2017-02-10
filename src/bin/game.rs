extern crate piston_window;
extern crate find_folder;
extern crate opengl_graphics;
extern crate image as oimage;

use piston_window::{PistonWindow, OpenGL, WindowSettings, Flip, Input, Context, clear, color, image, EventLoop, polygon, Transformed};
use opengl_graphics::{GlGraphics,TextureSettings};
use opengl_graphics::Texture;
use piston_window::rectangle::square;
use oimage::{DynamicImage, RgbaImage, GenericImage};
use std::path::Path;

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
        match e {
            Input::Render(args) => {
                gl.draw(args.viewport(),|c, g| app.render(c, g))
            }, 
            _ => {}
        }
    }
}

struct App{
    background : Texture,
    image: Vec<Texture>,
}

const POLYGON: &'static [[f64; 2]] = &[
    [0.0, -8.0],
    [20.0, 0.0],
    [0.0, 8.0]
];

impl App {
    fn render(&mut self, c: Context, g: &mut GlGraphics){
        clear(color::BLACK, g);
        image(&self.background, c.transform, g);
        // image.draw(&background, default_draw_state(), c.transform, g);
        image(&self.image[0], c.transform.zoom(0.5), g);
    }

    fn new(background: Texture, origin_image: &mut RgbaImage) -> App{
        App{
            background: background,
            image: App::splite_single_chess(origin_image),
        }        
    }

    fn splite_single_chess(origin_image: &mut RgbaImage) -> Vec<Texture>{
        let mut vec = vec![];
        let mut sub_image = origin_image.sub_image(29, 61, 120, 120);
        let (x, y) = sub_image.dimensions();
        let (a,b,c,d) = sub_image.bounds();

        println!("x: {}, y: {}", x, y);
        println!("a: {}, b: {}, c: {}, d: {} ", a, b, c, d);
        vec.push(Texture::from_image(&sub_image.to_image(), &TextureSettings::new()));
        vec
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

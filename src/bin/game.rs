extern crate piston_window;
extern crate find_folder;


use piston_window::*;

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
    let rust_logo = assets.join("timg.jpg");

    let rust_logo = Texture::from_path(
            &mut window.factory,
            &rust_logo,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    
    
    
    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            image(&rust_logo, c.transform, g);
        });
    }
}

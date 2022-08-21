// Draw lines based on mouse position and events
extern crate image as im;
extern crate piston_window;
extern crate vecmath;

use piston_window::*;
// use vecmath::*;
use rand::Rng;
use rusticate::plot_line;

fn main() {
    let mut rng = rand::thread_rng();
    let opengl = OpenGL::V3_2;
    let (width, height) = (300, 300);
    let mut window: PistonWindow = WindowSettings::new("piston: paint", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

    let mut last_pos: Option<[f64; 2]> = None;
    let mut start_pos: Option<[f64; 2]> = None;
    let mut end_pos: Option<[f64; 2]> = None;

    while let Some(e) = window.next() {
        if e.render_args().is_some() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
        if let Some(pos) = e.mouse_cursor_args() {
            last_pos = Some([pos[0], pos[1]]);
        }
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                if let Some(lp) = last_pos {
                    start_pos = Some([lp[0], lp[1]]);
                }
            }
        };
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                if let Some(lp) = last_pos {
                    end_pos = Some([lp[0], lp[1]]);
                }
                if let (Some(sp), Some(ep)) = (start_pos, end_pos) {
                    // println!("spx {} spy {} epx {} epy {}", sp[0] as i32, sp[1] as i32, ep[0] as i32, ep[1] as i32);
                    let red = rng.gen::<u8>();
                    let green = rng.gen::<u8>();
                    let blue = rng.gen::<u8>();
                    for pix in plot_line(sp[0] as i32, sp[1] as i32, ep[0] as i32, ep[1] as i32) {
                        canvas.put_pixel(pix.0, pix.1, im::Rgba([red, green, blue, 255]));
                    }
                }
            }
        }
    }
}

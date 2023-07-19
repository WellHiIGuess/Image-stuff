mod image;

use raylib::prelude::*;
use image::MyImage;

fn main() {
    let mut image = MyImage::new(10);
    image.start();

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .resizable()
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        image.draw(&mut d);

        if d.is_key_down(KeyboardKey::KEY_UP) {
            image.zoom += 1.0 * d.get_frame_time();
        } else if d.is_key_down(KeyboardKey::KEY_DOWN) {
            image.zoom -= 1.0 * d.get_frame_time();
        }
    }
}

use raylib::prelude::*;
#[allow(unused_imports)]
use raylib::*;
const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

struct Imagepos {
    position: Vector2,
    speed: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("my raylib app")
        .resizable()
        .vsync()
        .build();

    let mut imagepos = Imagepos {
        position: Vector2 { x: 307.0, y: 442.0 },
        speed: 5.0,
    };

    let images = rl.load_texture(&thread, "images/Spaceship1.png").unwrap();

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) | rl.is_key_down(KeyboardKey::KEY_D) {
            imagepos.position.x += imagepos.speed;
            if imagepos.position.x > SCREEN_WIDTH - 40.0 {
                imagepos.position.x = SCREEN_WIDTH - 40.0;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) | rl.is_key_down(KeyboardKey::KEY_A) {
            imagepos.position.x -= imagepos.speed;

            if imagepos.position.x < 4.0 {
                imagepos.position.x = 4.0;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) | rl.is_key_down(KeyboardKey::KEY_W) {
            imagepos.position.y -= imagepos.speed;

            //   if imagepos.position.y < SCREEN_HEIGHT - 80.0 {
            //     imagepos.position.y = SCREEN_HEIGHT - 80.0;
            //};
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) | rl.is_key_down(KeyboardKey::KEY_S) {
            imagepos.position.y += imagepos.speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_ESCAPE) {
            rl.window_should_close();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_text("RAYLIB GRAPHICS LIBRARY ", 12, 12, 24, Color::BLACK);

        d.draw_texture_v(&images, imagepos.position, Color::WHITE);

        println!("{:?}", d.get_mouse_position())
    }
}

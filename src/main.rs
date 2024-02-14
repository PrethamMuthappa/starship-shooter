use raylib::prelude::*;
#[allow(unused_imports)]
use raylib::*;
const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 690.0;

struct Imagepos {
    position: Vector2,
    speed: f32,
}

struct LazerLines {
    start_pos_x: i32,
    star_pos_y: i32,
    end_pos_x: i32,
    end_pos_y: i32,
    color: Color,
    speed: i32,
}

struct BackgroudImage {
    pos: Vector2,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("my raylib app")
        .resizable()
        .vsync()
        .build();

    let mut imagepos = Imagepos {
        position: Vector2 { x: 512.0, y: 645.0 },
        speed: 5.0,
    };

    let mut backs = BackgroudImage {
        pos: Vector2 { x: 0.0, y: 00.0 },
    };

    let mut lazers = LazerLines {
        start_pos_x: 512,
        star_pos_y: 645,
        end_pos_x: 512,
        end_pos_y: 600,
        color: Color::RED,
        speed: 10,
    };

    let mut activated: bool = false;
    let back = rl.load_texture(&thread, "images/p.png").unwrap();
    let images = rl.load_texture(&thread, "images/Spaceship1.png").unwrap();
    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) | rl.is_key_down(KeyboardKey::KEY_D) {
            imagepos.position.x += imagepos.speed;
            lazers.start_pos_x = imagepos.position.x as i32;

            if imagepos.position.x > SCREEN_WIDTH - 40.0 {
                imagepos.position.x = SCREEN_WIDTH - 40.0;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) | rl.is_key_down(KeyboardKey::KEY_A) {
            imagepos.position.x -= imagepos.speed;
            lazers.start_pos_x = imagepos.position.x as i32;

            if imagepos.position.x < 4.0 {
                imagepos.position.x = 4.0;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) | rl.is_key_down(KeyboardKey::KEY_W) {
            imagepos.position.y -= imagepos.speed;
            lazers.star_pos_y = imagepos.position.y as i32;

            if imagepos.position.y < 5.0 {
                imagepos.position.y = 5.0
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) | rl.is_key_down(KeyboardKey::KEY_S) {
            imagepos.position.y += imagepos.speed;
            lazers.end_pos_y = imagepos.position.y as i32;

            if imagepos.position.y > 645.0 {
                imagepos.position.y = 645.0;
            }
        }

        if rl.is_key_down(KeyboardKey::KEY_ESCAPE) {
            rl.window_should_close();
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            activated = true;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_text("RAYLIB GRAPHICS LIBRARY ", 12, 12, 24, Color::BLACK);

        d.draw_texture_v(&back, backs.pos, Color::WHITE);
        d.draw_texture_v(&images, imagepos.position, Color::WHITE);

        if activated == true {
            d.draw_line(
                lazers.start_pos_x,
                lazers.star_pos_y,
                lazers.start_pos_x,
                lazers.star_pos_y - 30,
                lazers.color,
            );
            // start the lazers to  move till end of screen

            lazers.star_pos_y -= lazers.speed;
            lazers.end_pos_y -= lazers.speed;
        };

        //  println!("{:?}", d.get_mouse_position());
        println!("start x {:?}", lazers.start_pos_x);
        println!(" end pos x{:?}", lazers.end_pos_x);
        // println!("{:?}", lazers.start_pos_x);
        // println!("{:?}", lazers.start_pos_x);
    }
}

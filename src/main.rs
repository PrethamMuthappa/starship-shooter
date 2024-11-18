use rand::*;
use raylib::prelude::*;
#[allow(unused_imports)]
use raylib::*;
const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 690.0;

struct Imagepos {
    position: Vector2,
    speed: f32,
    color: Color,
}

#[allow(dead_code)]
struct LazerLines {
    start_pos_x: i32,
    star_pos_y: i32,
    end_pos_x: i32,
    end_pos_y: i32,
    color: Color,
    speed: i32,
}

struct BackgroudImage {
    pos1: Vector2,
    pos2: Vector2,
    color: Color,
    speed: f32,
}

struct Asteiods {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    speed: f32,
}

#[derive(Debug)]
struct ScoreCounter {
    score:i32,
}
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("my raylib app")
        .vsync()
        .build();

    let mut imagepos = Imagepos {
        position: Vector2 { x: 476.0, y: 611.0 },
        speed: 5.0,
        color: Color::WHITE,
    };

    let mut backs = BackgroudImage {
        pos1: Vector2 { x: 0.0, y: 0.0 },
        pos2: Vector2 { x: 0.0, y: -SCREEN_HEIGHT },
        color: Color::WHITE,
        speed: 5.0,
    };

    let mut lazers = LazerLines {
        start_pos_x: 512,
        star_pos_y: 645,
        end_pos_x: 512,
        end_pos_y: 600,
        color: Color::RED,
        speed: 11,
    };

    let mut asteroids_vec: Vec<Asteiods> = Vec::new();
    let mut rects_vec: Vec<Rectangle> = Vec::new();
    
    for _ in 0..5 {  // Start with 5 asteroids
        let asteroid = Asteiods {
            x: rand::thread_rng().gen_range(0.0..SCREEN_WIDTH),
            y: rand::thread_rng().gen_range(-600.0..0.0), // Stagger initial positions
            width: rand::thread_rng().gen_range(20.0..35.0),
            height: rand::thread_rng().gen_range(20.0..35.0),
            speed: rand::thread_rng().gen_range(2.0..5.0), // Random speeds
        };
        
        let rect = Rectangle::new(asteroid.x, asteroid.y, asteroid.width, asteroid.height);
        
        asteroids_vec.push(asteroid);
        rects_vec.push(rect);
    }

    let mut activated: bool = false;
    let back = rl.load_texture(&thread, "images/space.png").unwrap();
    let images = rl.load_texture(&thread, "images/ship0.png").unwrap();
    let enemy = rl.load_texture(&thread, "images/enemy_3.png").unwrap();
    let mut game_over = false;

    let mut score_counter=ScoreCounter{score:0};

    while !rl.window_should_close() {
        
        if !game_over {
        score_counter.score+=1;
        }

     
        backs.pos1.y += backs.speed;
        backs.pos2.y += backs.speed;

        if backs.pos1.y >= SCREEN_HEIGHT {
            backs.pos1.y = -SCREEN_HEIGHT;
        }

        if backs.pos2.y >= SCREEN_HEIGHT {
            backs.pos2.y = -SCREEN_HEIGHT;
        }

        if !game_over {
            if rl.is_key_down(KeyboardKey::KEY_RIGHT) | rl.is_key_down(KeyboardKey::KEY_D) {
                imagepos.position.x += imagepos.speed;
                lazers.start_pos_x = imagepos.position.x as i32;

                if imagepos.position.x > SCREEN_WIDTH - 90.0 {
                    imagepos.position.x = SCREEN_WIDTH - 90.0;
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

                if imagepos.position.y > 611.0 {
                    imagepos.position.y = 611.0;
                }
            }
        }

        if rl.is_key_down(KeyboardKey::KEY_ESCAPE) {
            rl.window_should_close();
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            activated = true;
        }

        if rl.is_window_ready() {
            // Update all asteroids
            for i in 0..asteroids_vec.len() {
                asteroids_vec[i].y += asteroids_vec[i].speed;
                rects_vec[i].x = asteroids_vec[i].x;
                rects_vec[i].y = asteroids_vec[i].y;

                // Reset asteroid when it goes off screen
                if asteroids_vec[i].y > SCREEN_HEIGHT {
                    asteroids_vec[i].y = 0.0;
                    asteroids_vec[i].x = rand::thread_rng().gen_range(0.0..SCREEN_WIDTH);
                    rects_vec[i].x = asteroids_vec[i].x;
                    rects_vec[i].y = asteroids_vec[i].y;
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_texture_v(&back, backs.pos1, backs.color);
        d.draw_texture_v(&back, backs.pos2, backs.color);
        d.draw_texture_v(&images, imagepos.position, imagepos.color);
        d.draw_texture(&enemy, 112, 80, Color::WHITE);
        d.draw_text(&score_counter.score.to_string(), 35, 10, 20, Color::WHITE);
        
        // Draw all asteroids
        for rect in &rects_vec {
            d.draw_rectangle_rec(rect, Color::RED);
        }

        // Create a rectangle for the player ship
        let player_rect = Rectangle::new(
            imagepos.position.x, 
            imagepos.position.y, 
            80.0,  // adjust these values based on your ship's size
            80.0
        );

        // Check collisions with all asteroids
        for rect in &rects_vec {
            if player_rect.check_collision_recs(rect) {
                game_over = true;
            }
        }

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

        if game_over {
            d.draw_text(
                &format!("GAME OVER! Press ESC to exit: Score: {}", score_counter.score), 
                (SCREEN_WIDTH/2.0 - 150.0) as i32, 
                (SCREEN_HEIGHT/2.0) as i32, 
                20, 
                Color::WHITE
            );

            backs.speed = 0.0;
        }

        println!("{:?}", d.get_mouse_position());
    }
}

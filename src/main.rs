use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Bouncing Ball")
        .vsync()
        .resizable()
        .build();

    rl.set_target_fps(60);

    let mut ball = Ball {
        position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        speed: Vector2::new(6.0, 5.0),
        radius: 50.0,
    };

    let mut pause = false;
    let frame_count = 0;

    while !rl.window_should_close() {
        /* UPDATE */
        if rl.is_key_pressed(KEY_SPACE) {
            pause = !pause;
        }
        if !pause {
            ball.position += ball.speed;

            if ball.position.x >= SCREEN_WIDTH - ball.radius || ball.position.x <= ball.radius {
                ball.speed.x *= -1.0;
            }
            if ball.position.y >= SCREEN_HEIGHT - ball.radius || ball.position.y <= ball.radius {
                ball.speed.y *= -1.0;
            }
        }

        /* DRAW */
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_circle(
            ball.position.x as i32,
            ball.position.y as i32,
            ball.radius,
            Color::MAROON,
        );
        d.draw_text(
            "Press space to pause ball movement",
            20,
            (SCREEN_HEIGHT as i32) - 40,
            30,
            Color::WHITE,
        );

        if pause && (frame_count / 30) % 2 == 0 {
            d.draw_text(
                "PAUSED",
                (SCREEN_WIDTH as i32) / 2 - 90,
                (SCREEN_HEIGHT as i32) / 2 - 25,
                50,
                Color::GRAY,
            );
        }

        d.draw_fps(10, 10);
    }
}

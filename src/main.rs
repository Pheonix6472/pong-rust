use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);
    let mut player = Paddle {
        rect: Rect::new(30.0, screen_height() / 2.0, 15.0, 150.0),
        speed: 150.0,
    };
    let mut ai = Paddle {
        rect: Rect::new(screen_width() - 30.0, screen_height() / 2.0, 15.0, 150.0),
        speed: 150.0,
    };

    let mut ball = Ball::new(
        screen_width() / 2.0,
        screen_height() / 2.0,
        10.0,
        ran_ball_vel(250.0),
    );
    let mut score_player = 0;
    let mut score_ai = 0;

    loop {
        clear_background(BLACK);
        let time = format!("{:.1}", get_time());

        let dt = get_frame_time();
        if is_key_down(KeyCode::W) {
            player.move_by(-player.speed * dt)
        };
        if is_key_down(KeyCode::S) {
            player.move_by(player.speed * dt)
        };
        if ball.circ.y < ai.rect.y + 75.0 {
            ai.move_by(-player.speed * dt)
        };
        if ball.circ.y > ai.rect.y + 75.0 {
            ai.move_by(player.speed * dt)
        };

        match ball.move_by(ball.speed.x * dt, ball.speed.y * dt) {
            BallEvent::HitLeft => {
                score_ai += 1;
                ball.ball_reset(250.0);
            }
            BallEvent::HitRight => {
                score_player += 1;
                ball.ball_reset(250.0);
            }
            BallEvent::None => {}
        }

        let score = format! {"{} : {}", score_player, score_ai};
        ball.paddle_col(&player);
        ball.paddle_col(&ai);
        player.draw();
        ai.draw();
        ball.draw();
        draw_text(
            &score,
            (screen_width() - measure_text(&score, None, 30.0 as u16, 1.0).width) / 2.0,
            40.0,
            30.0,
            BLUE,
        );
        draw_text(
            &time,
            (screen_width() - measure_text(&time, None, 15.0 as u16, 1.0).width) / 2.0,
            screen_height() - 40.0,
            15.0,
            WHITE,
        );
        next_frame().await;
    }
}

struct Paddle {
    rect: Rect,
    speed: f32,
}

impl Paddle {
    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
    }

    fn move_by(&mut self, speed: f32) {
        self.rect.y += speed;
        self.rect.y = self.rect.y.clamp(0.0, screen_height() - self.rect.h);
    }
}

enum BallEvent {
    None,
    HitLeft,
    HitRight,
}
struct Ball {
    circ: Circle,
    speed: Vec2,
}

impl Ball {
    fn new(x: f32, y: f32, radius: f32, speed: Vec2) -> Self {
        Self {
            circ: Circle::new(x, y, radius),
            speed,
        }
    }

    fn draw(&self) {
        draw_circle(self.circ.x, self.circ.y, self.circ.radius(), WHITE);
    }

    fn move_by(&mut self, dx: f32, dy: f32) -> BallEvent {
        self.circ.x += dx;
        self.circ.y += dy;
        let r = self.circ.radius();
        if self.circ.y <= r || self.circ.y >= screen_height() - r {
            self.circ.y = self.circ.y.clamp(r, screen_height() - r);
            self.speed.y *= -1.0;
        }

        if self.circ.x <= r {
            BallEvent::HitLeft
        } else if self.circ.x >= screen_width() - r {
            BallEvent::HitRight
        } else {
            BallEvent::None
        }
    }

    fn paddle_col(&mut self, paddle: &Paddle) {
        if self.circ.overlaps_rect(&paddle.rect) {
            self.speed.x *= -1.0;
            if self.speed.x > 0.0 {
                self.circ.x = paddle.rect.x + paddle.rect.w + self.circ.radius();
            } else {
                self.circ.x = paddle.rect.x - self.circ.radius();
            }
        }
    }

    fn ball_reset(&mut self, speed: f32) {
        self.circ.x = screen_width() / 2.0;
        self.circ.y = screen_height() / 2.0;
        self.speed = ran_ball_vel(speed);
    }
}

fn ran_ball_vel(speed: f32) -> Vec2 {
    let angle = rand::gen_range(-45.0_f32, 45.0_f32).to_radians();
    let dir_x = if rand::gen_range(0, 2) == 0 {
        -1.0
    } else {
        1.0
    };
    vec2(angle.cos() * speed * dir_x, angle.sin() * speed)
}

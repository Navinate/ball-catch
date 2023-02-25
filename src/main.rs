use macroquad::prelude::*;

struct Paddle {
    position: f32,
    color: macroquad::color::Color,
}

impl Paddle {
    pub fn new() -> Self {
        Self {
            position: 0.0,
            color: RED,
        }
    }

    pub fn render_paddle(&self) {
        draw_rectangle(
            self.position * (100.0) + screen_width() / 4.0 - 50.0,
            500.0,
            100.0,
            25.0,
            self.color,
        )
    }

    pub fn update_paddle(&mut self) {
        let direction = match (
            is_key_released(KeyCode::Left),
            is_key_released(KeyCode::Right),
        ) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.position += direction;
        if self.position > 4.0 {
            self.position = 4.0;
        } else if self.position < 0.0 {
            self.position = 0.0;
        }
    }
}

struct Ball {
    position: f32,
    color: macroquad::color::Color,
    is_falling: bool,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            position: 0.0,
            color: GREEN,
            is_falling: false,
        }
    }

    pub fn spawn() {}

    pub fn render_ball(&self, dt: f32) {
        if self.is_falling {
            let x = dt - 3.0;
            draw_circle(
                self.position * (100.0) + screen_width() / 4.0 - 25.0,
                -1.0 * x * x + 10.0,
                50.0,
                self.color,
            )
        }
    }
}

#[macroquad::main("Ball Catch")]
async fn main() {
    let mut paddle = Paddle::new();

    loop {
        clear_background(BLUE);
        // render_ball();
        // render_wall();
        paddle.update_paddle();
        paddle.render_paddle();
        // check_for_hit();

        next_frame().await
    }
}

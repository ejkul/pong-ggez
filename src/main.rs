use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameError, GameResult};

mod ball;
mod paddle;

use crate::ball::Ball;
use crate::paddle::*;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const BALL_SIZE: f32 = 10.0;
pub const PADDLE_SIZE: [f32; 2] = [20.0, 60.0];
pub const PADDLE_ACC: f32 = 5.0;
pub const PADDLE_MAX_VEL: f32 = 5.0;

fn main() -> GameResult {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Pong", "Andrzej")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = Pong::new(&mut ctx)?;

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => {
            println!("Exited cleanly.");
            Ok(())
        }
        Err(e) => {
            println!("Error occured: {}", e);
            Err(GameError::from(e))
        }
    }
}

pub enum State {
    Serve,
    Play,
    Won,
}

struct Pong {
    // Your state here...
    ball: Ball,
    paddles: [Paddle; 2],
}

impl Pong {
    pub fn new(_ctx: &mut Context) -> GameResult<Pong> {
        // Load/create resources such as images here.
        let ball = Ball::new();
        let paddle1 = Paddle::new(Side::Left)?;
        let paddle2 = Paddle::new(Side::Right)?;
        Ok(Pong {
            ball,
            paddles: [paddle1, paddle2],
        })
    }
}

impl EventHandler for Pong {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.ball.collides()?;
        self.ball.update()?;
        self.paddles[0].update()?;
        self.paddles[1].update()?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        self.ball.draw(ctx)?;
        self.paddles[0].draw(ctx)?;
        self.paddles[1].draw(ctx)?;
        graphics::present(ctx)
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: ggez::event::KeyCode,
        _keymods: ggez::event::KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            match keycode {
                ggez::event::KeyCode::W => {
                    self.paddles[0].vel.y = std::cmp::min(
                        (self.paddles[0].vel.y - PADDLE_ACC) as i32,
                        -PADDLE_MAX_VEL as i32,
                    ) as f32
                }
                ggez::event::KeyCode::S => {
                    self.paddles[0].vel.y = std::cmp::max(
                        (self.paddles[0].vel.y + PADDLE_ACC) as i32,
                        PADDLE_MAX_VEL as i32,
                    ) as f32
                }
                ggez::event::KeyCode::Up => {
                    self.paddles[1].vel.y = std::cmp::min(
                        (self.paddles[1].vel.y - PADDLE_ACC) as i32,
                        -PADDLE_MAX_VEL as i32,
                    ) as f32
                }
                ggez::event::KeyCode::Down => {
                    self.paddles[1].vel.y = std::cmp::max(
                        (self.paddles[1].vel.y + PADDLE_ACC) as i32,
                        PADDLE_MAX_VEL as i32,
                    ) as f32
                }
                _ => (),
            }
        }
    }
    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: ggez::event::KeyCode,
        _keymods: ggez::event::KeyMods,
    ) {
        match keycode {
            ggez::event::KeyCode::W => self.paddles[0].vel.y = 0.0,
            ggez::event::KeyCode::S => self.paddles[0].vel.y = 0.0,
            ggez::event::KeyCode::Up => self.paddles[1].vel.y = 0.0,
            ggez::event::KeyCode::Down => self.paddles[1].vel.y = 0.0,
            _ => (),
        }
    }
}

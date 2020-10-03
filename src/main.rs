use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameError, GameResult};

mod ball;
mod paddle;

use crate::ball::Ball;
use crate::paddle::*;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const BALL_SIZE: f32 = 10.0;
pub const BALL_SPEED: f32 = 5.;
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
#[derive(Clone, Copy)]
pub enum State {
    Serve,
    Play,
    Won,
}

struct Pong {
    // Your state here...
    ball: Ball,
    paddles: [Paddle; 2],
    score: [u32; 2],
    state: State,
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
            score: [0, 0],
            state: State::Serve,
        })
    }
}

impl EventHandler for Pong {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, 60) {
            match self.state {
                State::Play => self.play_state(ctx)?,
                _ => println!("Unknown stat"),
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.ball.draw(ctx)?;
        self.paddles[0].draw(ctx)?;
        self.paddles[1].draw(ctx)?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: ggez::event::KeyCode,
        _keymods: ggez::event::KeyMods,
        _repeat: bool,
    )  {
        match (self.state, keycode) {
            (State::Serve, ggez::event::KeyCode::Space) => {
                self.ball.serve();
                self.state = State::Play;
            }
            _ => println!("Unknown Input"),
        }
    }
}

impl Pong {
    fn handle_input(&mut self, ctx: &mut Context) -> GameResult {
        if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::W) {
            self.paddles[0].vel.y = -PADDLE_MAX_VEL;
        } else if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::S) {
            self.paddles[0].vel.y = PADDLE_MAX_VEL;
        } else {
            self.paddles[0].vel.y = 0.;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::Up) {
            self.paddles[1].vel.y = -PADDLE_MAX_VEL;
        } else if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::Down) {
            self.paddles[1].vel.y = PADDLE_MAX_VEL;
        } else {
            self.paddles[1].vel.y = 0.;
        }
        Ok(())
    }
    fn handle_goal(&mut self) {
        if self.ball.loc.x > WINDOW_WIDTH {
            self.score[0] += 1;
            self.ball.reset();
            self.paddles[0].reset();
            self.paddles[1].reset();
            self.state = State::Serve;
        }
        if self.ball.loc.x < 0.{
            self.score[1] += 1;
            self.ball.reset();
            self.paddles[0].reset();
            self.paddles[1].reset();
            self.state = State::Serve;
        }
    }
    fn play_state(&mut self, ctx: &mut Context) -> GameResult {
        self.handle_input(ctx)?;
        self.ball.collides_wall()?;
        self.paddles[0].update()?;
        self.paddles[1].update()?;
        self.ball.collides_paddle(self.paddles[1])?;
        self.ball.collides_paddle(self.paddles[0])?;
        self.ball.update()?;
        self.handle_goal();
        println!("${:?}", self.score);
        Ok(())
    }
}

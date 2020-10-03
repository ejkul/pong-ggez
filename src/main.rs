use ::std::path;
use ggez::event::{self, EventHandler};
use ggez::nalgebra as na;
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
pub const PADDLE_MAX_VEL: f32 = 10.0;
pub const WIN_POINTS: u32 = 10;

fn main() -> GameResult {
    // Make a Context.

    let assets_path = path::PathBuf::from("../assets");

    let (mut ctx, mut event_loop) = ContextBuilder::new("Pong", "Andrzej")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
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
    Win,
}

const FONT: [u8; 13652] = *include_bytes!("../assets/font/square.ttf");

struct Pong {
    // Your state here...
    ball: Ball,
    paddles: [Paddle; 2],
    score: [u32; 2],
    state: State,
    scoreboard: [graphics::Text; 2],
}

fn parse_score(ctx: &mut Context, num: u32) -> GameResult<graphics::Text> {
    Ok(graphics::Text::new(
        graphics::TextFragment::new(String::from(num.to_string()))
            .color(graphics::BLACK)
            .scale(graphics::Scale::uniform(30.0))
            .font(graphics::Font::new_glyph_font_bytes(ctx, &FONT)?),
    ))
}

fn ui_text(ctx: &mut Context, text: &str, scale: f32) -> GameResult<graphics::Text> {
    Ok(graphics::Text::new(
        graphics::TextFragment::new(text)
            .color(graphics::BLACK)
            .scale(graphics::Scale::uniform(scale))
            .font(graphics::Font::new_glyph_font_bytes(ctx, &FONT)?),
    ))
}

impl Pong {
    pub fn new(ctx: &mut Context) -> GameResult<Pong> {
        // Load/create resources such as images here.
        let ball = Ball::new();
        let paddle1 = Paddle::new(Side::Left)?;
        let paddle2 = Paddle::new(Side::Right)?;
        Ok(Pong {
            ball,
            paddles: [paddle1, paddle2],
            score: [0, 0],
            state: State::Serve,
            scoreboard: [parse_score(ctx, 0)?, parse_score(ctx, 0)?],
        })
    }
}

impl EventHandler for Pong {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, 60) {
            match self.state {
                State::Play => self.play_state(ctx)?,
                _ => (),
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.ball.draw(ctx)?;
        self.paddles[0].draw(ctx)?;
        self.paddles[1].draw(ctx)?;
        graphics::draw(ctx, &self.scoreboard[0], (na::Point2::new(30.0, 30.0),))?;
        graphics::draw(
            ctx,
            &self.scoreboard[1],
            (na::Point2::new(WINDOW_WIDTH - 30.0, 30.0),),
        )?;
        match self.state {
            State::Win => {
                let won: &str = if self.score[0] > self.score[1] { "Player 1"} else {"Player 2"};
                let text: graphics::Text = ui_text(ctx, &(won.to_owned() + " Won"), 100.0)?;
                let text_dim: (u32, u32) = text.dimensions(ctx);
                graphics::draw(
                    ctx,
                    &text,
                    (na::Point2::new(
                        WINDOW_WIDTH / 2.0 - text_dim.0 as f32 / 2.0,
                        WINDOW_HEIGHT / 2.0 - text_dim.1 as f32 * 2.0,
                    ),),
                )?;
            }
            State::Serve => {
                let text: graphics::Text = ui_text(ctx, "Press Space to serve", 50.0)?;
                let text_dim: (u32, u32) = text.dimensions(ctx);
                graphics::draw(
                    ctx,
                    &text,
                    (na::Point2::new(
                        WINDOW_WIDTH / 2.0 - text_dim.0 as f32 / 2.0,
                        WINDOW_HEIGHT / 2.0 + text_dim.1 as f32 * 2.0,
                    ),),
                )?;
            }
            _ => (),
        }
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: ggez::event::KeyCode,
        _keymods: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        match (self.state, keycode) {
            (State::Serve, ggez::event::KeyCode::Space) => {
                self.ball.serve();
                self.state = State::Play;
            }
            (State::Win, ggez::event::KeyCode::Space) => {
                match self.reset() {
                    Ok(_) => (),
                    Err(_) => (),
                };
                self.state = State::Serve;
            }
            _ => (),
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
    fn reset(&mut self) -> GameResult {
        self.ball.reset()?;
        self.paddles[0].reset();
        self.paddles[1].reset();
        self.state = State::Serve;
        Ok(())
    }
    fn handle_goal(&mut self, ctx: &mut Context) -> GameResult {
        if self.ball.loc.x > WINDOW_WIDTH {
            self.score[0] += 1;
            self.scoreboard[0] = parse_score(ctx, self.score[0])?;
            self.reset()?;
        }
        if self.ball.loc.x < 0. {
            self.score[1] += 1;
            self.scoreboard[1] = parse_score(ctx, self.score[1])?;
            self.reset()?;
        }
        if self.score[0] == WIN_POINTS || self.score[1] == WIN_POINTS {
            self.state = State::Win;
        }
        Ok(())
    }

    fn play_state(&mut self, ctx: &mut Context) -> GameResult {
        self.handle_input(ctx)?;
        self.ball.collides_wall()?;
        self.paddles[0].update()?;
        self.paddles[1].update()?;
        self.ball.collides_paddle(self.paddles[1])?;
        self.ball.collides_paddle(self.paddles[0])?;
        self.ball.update()?;
        self.handle_goal(ctx)?;
        println!("${:?}", self.score);
        Ok(())
    }
}

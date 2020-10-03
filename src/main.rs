use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

mod ball;

use crate::ball::Ball;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const BALL_SIZE: f32 = 10.0;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = Pong::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Pong {
    // Your state here...
    ball: Ball,
}

impl Pong {
    pub fn new(_ctx: &mut Context) -> Pong {
        // Load/create resources such as images here.
        let ball = Ball::new();
        Pong { ball }
    }
}

impl EventHandler for Pong {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.ball.update()?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        self.ball.draw(ctx)?;
        graphics::present(ctx)
    }
}

use crate::{BALL_SIZE, PADDLE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use ggez::graphics::Mesh;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameError, GameResult};

pub struct Paddle {
    pub loc: Point2<f32>,
    pub vel: Vector2<f32>,
}

pub enum Side {
    Left,
    Right,
}

impl Paddle {
    pub fn new(side: Side) -> GameResult<Paddle> {
        match side {
            Side::Left => Ok(Paddle {
                loc: Point2::new(0.0, WINDOW_HEIGHT / 2.0 - PADDLE_SIZE[1] / 2.0),
                vel: Vector2::new(0.0, 0.0),
            }),
            Side::Right => Ok(Paddle {
                loc: Point2::new(
                    WINDOW_WIDTH - PADDLE_SIZE[0],
                    WINDOW_HEIGHT / 2.0 - PADDLE_SIZE[1] / 2.0,
                ),
                vel: Vector2::new(0.0, 0.0),
            }),
        }
    }
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let paddle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, PADDLE_SIZE[0], PADDLE_SIZE[1]),
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &paddle, (self.loc,))?;
        Ok(())
    }
}

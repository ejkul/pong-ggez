use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};
use ggez::graphics::{Mesh};
use ggez::{Context, GameResult, GameError, graphics};
use ggez::nalgebra::{Point2, Vector2};
pub struct Ball{
    pub loc: Point2<f32>,
    pub vel: Vector2<f32>,
}

impl Ball {
    pub fn new () -> Ball{
        Ball {
            loc: Point2::new(WINDOW_WIDTH/2.0,WINDOW_HEIGHT/2.0),
            vel: Vector2::new(0.0,0.0)
        }
    }
    pub fn draw (&self, ctx: &mut Context) -> GameResult<()> {
        let circle = Mesh::new_circle(ctx,graphics::DrawMode::fill(), self.loc, 10.0, 2.0, graphics::BLACK)?;
        graphics::draw(ctx, &circle, (self.loc,))
    }
}
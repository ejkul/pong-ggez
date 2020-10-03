use crate::{BALL_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use ggez::graphics::Mesh;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameError, GameResult};
pub struct Ball {
    pub loc: Point2<f32>,
    pub vel: Vector2<f32>,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            loc: Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            vel: Vector2::new(5.0, 5.0),
        }
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let circle = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::origin(),
            BALL_SIZE,
            1.0,
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &circle, (self.loc,))?;
        Ok(())
    }
    pub fn update(&mut self) -> GameResult<()> {
        self.loc = self.loc + self.vel;
        self.collides();
        Ok(())
    }
    pub fn collides(&mut self) {
        // checl wall collision
        if self.loc.y >= WINDOW_HEIGHT - BALL_SIZE {
            self.vel.y = self.vel.y * -1.0;
            self.loc.y = WINDOW_HEIGHT - BALL_SIZE;
        } else if self.loc.y <= 0.0 {
            self.vel.y = self.vel.y * -1.0;
            self.loc.y = 0.0;
        }
    }
}

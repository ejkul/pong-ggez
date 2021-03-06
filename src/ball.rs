use crate::{Paddle, BALL_SIZE, BALL_SPEED, PADDLE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use ggez::graphics::Mesh;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameError, GameResult};
use rand::Rng;

pub struct Ball {
    pub loc: Point2<f32>,
    pub vel: Vector2<f32>,
    pub rng: rand::ThreadRng,
}

impl Ball {
    pub fn new() -> Ball {
        let mut rng = rand::thread_rng();
        println!("${:?}", rng.gen::<f32>());
        Ball {
            loc: Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            vel: Vector2::new(0.0, 0.0),
            rng,
        }
    }
    pub fn reset(&mut self) -> GameResult {
        self.loc = Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        self.vel = Vector2::new(0.0, 0.0);
        Ok(())
    }
    pub fn serve(&mut self) {
        self.vel.x = if self.rng.gen::<f32>() > 0.5 {
            -BALL_SPEED
        } else {
            BALL_SPEED
        };
        self.vel.y = self.rng.gen_range(-BALL_SPEED/2., BALL_SPEED/2.);
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
        Ok(())
    }
    pub fn collides_wall(&mut self) -> GameResult<()> {
        // checl wall collision
        if self.loc.y >= WINDOW_HEIGHT - BALL_SIZE {
            self.vel.y = self.vel.y * -1.0;
            self.loc.y = WINDOW_HEIGHT - BALL_SIZE;
        } else if self.loc.y <= 0.0 {
            self.vel.y = self.vel.y * -1.0;
            self.loc.y = 0.0;
        }
        Ok(())
    }
    pub fn collides_paddle(&mut self, paddle: Paddle) -> GameResult<()> {
        if !(self.loc.x > paddle.loc.x + PADDLE_SIZE[0] || paddle.loc.x > self.loc.x + BALL_SIZE)
            && !(self.loc.y > paddle.loc.y + PADDLE_SIZE[1]
                || paddle.loc.y > self.loc.y + BALL_SIZE)
        {
            self.vel.y = if self.vel.y > 0.0 {
                self.rng.gen_range(1., 5.)
            } else {
                -self.rng.gen_range(1., 5.)
            };
            self.loc.x = if self.vel.x > 0.0 {
                paddle.loc.x - 5.
            } else {
                paddle.loc.x + PADDLE_SIZE[0] + 5.
            };
            self.vel.x *= -1.05;
        }
        Ok(())
    }
}

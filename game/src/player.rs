use bracket_lib::prelude::*;

pub struct Player {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) velocity: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    pub(crate) fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    pub(crate) fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    pub(crate) fn flap(&mut self) {
        self.velocity = -2.0
    }
}

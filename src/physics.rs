use euclid::{Angle, Box2D, Rotation2D, UnknownUnit};

use crate::config::get_config;
pub type Vector = euclid::Vector2D<f32, UnknownUnit>;

pub struct Charge {
    pub pos: Vector,
    pub ch: f32,
}

impl Charge {
    fn f(&self, ch: &Charge) -> Vector {
        let v = ch.pos - self.pos;
        let r2 = v.square_length();
        v.normalize() * (ch.ch * self.ch) / r2
    }
}

pub struct Sheet {
    pub charges: Vec<Charge>,
}

impl Sheet {
    pub fn get_lines(&self) -> Vec<Vec<Vector>> {
        self.charges
            .iter()
            .flat_map(|ch| {
                let mut lines = vec![];

                let lines_count = get_config().lines_count;

                let rot = Rotation2D::new(Angle::<f32>::two_pi() / lines_count as f32);
                let mut v = Vector::one() * get_config().start_radius;
                for _ in 0..lines_count {
                    lines.push(self.get_line(ch.pos + v));
                    v = rot.transform_vector(v);
                }

                lines
            })
            .collect()
    }

    fn get_line(&self, start: Vector) -> Vec<Vector> {
        let mut lines = vec![];

        let mut q = Charge { ch: 1., pos: start };

        let bounding = Box2D::new(
            Vector::zero().to_point(),
            Vector::new(get_config().width as f32, get_config().height as f32).to_point(),
        );

        for _ in 0..get_config().max_iterations {
            lines.push(q.pos);
            let force = self
                .charges
                .iter()
                .map(|ch| ch.f(&q))
                .fold(Vector::zero(), |v1, v2| v1 + v2);
            q.pos += force * get_config().force_factor;

            if !bounding.contains(q.pos.to_point()) {
                break;
            }
        }

        lines
    }
}

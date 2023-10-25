use euclid::{Angle, Rotation2D};
use raqote::{DrawOptions, DrawTarget, SolidSource, PathBuilder, StrokeStyle};

enum M {}

type Vector = euclid::Vector2D<f32, M>;

struct Charge {
    pos: Vector,
    ch: f32,
}

impl Charge {
    fn f(&self, ch: &Charge) -> Vector {
        let v = ch.pos - self.pos;
        let r2 = v.square_length();
        v.normalize() * (ch.ch * self.ch) / r2
    }
}

struct Sheet {
    charges: Vec<Charge>,
}

impl Sheet {
    fn get_lines(&self) -> Vec<Vec<Vector>> {
        self.charges
            .iter()
            .flat_map(|ch| {
                let mut lines = vec![];

                let rot = Rotation2D::new(Angle::<f32>::frac_pi_3());
                // TODO: Magic Number
                let mut v = Vector::one() * 5.0;
                for _ in 0..6 {
                    lines.push(self.get_line(ch.pos + v));
                    v = rot.transform_vector(v);
                }

                lines
            })
            .collect()
    }

    fn get_line(&self, mut start: Vector) -> Vec<Vector> {
        let mut lines = vec![];

        let mut q = Charge {
            // TODO: Magic Number
            ch: 1.,
            pos: start,
        };

        // TODO: Magic Number
        for _ in 0..20_000 {
            lines.push(q.pos);
            let force = self
                .charges
                .iter()
                .map(|ch| ch.f(&q))
                .fold(Vector::zero(), |v1, v2| v1 + v2);
            // TODO: Magic Number
            q.pos += force * 100.
        }

        lines
    }
}

fn main() {
    let sheet = Sheet {
        charges: vec![
            Charge {
                ch: 10.0,
                pos: Vector::new(200.0, 350.0),
            },
            Charge {
                ch: 20.0,
                pos: Vector::new(600.0, 700.0),
            },
        ],
    };

    let mut dt = DrawTarget::new(1000, 1000);
    let white_solid = raqote::Source::Solid(SolidSource {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    });
    let black_solid = raqote::Source::Solid(SolidSource {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    });
    dt.fill_rect(0., 0., 1000., 1000., &white_solid, &DrawOptions::new());

    let mut pb = PathBuilder::new();

    sheet.get_lines().iter().for_each(|line| {
        let mut first = true;
        for p in line {
            if first {
                pb.move_to(p.x, p.y);
                first = false;
                continue;
            }
            pb.line_to(p.x, p.y);
        }
    });

    let path = pb.finish();
    dt.stroke(
        &path,
        &black_solid,
        &raqote::StrokeStyle {
            cap: raqote::LineCap::Round,
            join: raqote::LineJoin::Round,
            width: 3.,
            miter_limit: 1.,
            dash_array: vec![],
            dash_offset: 0.,
        },
        &DrawOptions::new()
    );
    
    dt.write_png("example.png");
}

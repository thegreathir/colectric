use config::get_config;
use physics::{Charge, Sheet, Vector};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource};

mod config;
mod physics;

fn main() {
    let sheet = Sheet {
        charges: vec![
            Charge {
                ch: 10.0,
                pos: Vector::new(200.0, 700.0),
            },
            Charge {
                ch: 50.0,
                pos: Vector::new(600.0, 250.0),
            },
        ],
    };

    let width = get_config().width as i32;
    let height = get_config().height as i32;

    let mut dt = DrawTarget::new(width, height);
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

    dt.fill_rect(
        0.,
        0.,
        width as f32,
        height as f32,
        &white_solid,
        &DrawOptions::new(),
    );

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
        &DrawOptions::new(),
    );

    dt.write_png("out.png").unwrap();
}

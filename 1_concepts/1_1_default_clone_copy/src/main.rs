#![feature(generic_const_exprs)]

#[derive(Copy, Default, Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
struct Polyline(Vec<Point>);

impl Polyline {
    fn new(p1: Point, p2: Point, rest: &[Point]) -> Self {
        let mut v = vec![p1, p2];
        v.extend_from_slice(&rest);
        Self(v)
    }
}

mod hack {
    pub trait Truth {}

    pub struct Wrap<const T: bool>;

    impl Truth for Wrap<true> {}
}

impl<const S: usize> From<[Point; S]> for Polyline
where
    hack::Wrap<{ S > 1 }>: hack::Truth,
{
    fn from(value: [Point; S]) -> Self {
        Self(Vec::from(value))
    }
}

impl TryFrom<Vec<Point>> for Polyline {
    type Error = &'static str;

    fn try_from(value: Vec<Point>) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            Err("must have at least 2 points")
        } else {
            Ok(Self(value))
        }
    }
}

fn main() {
    let a = Point::new(0., 1.);
    let b = Point::new(1., 0.);

    let line = Polyline::from([a, b]);
    // let line1 = Polyline::from([a]);
    let line2 = Polyline::try_from(vec![a, b]).unwrap();

    let line3 = Polyline::new(a, b, &[]);
}

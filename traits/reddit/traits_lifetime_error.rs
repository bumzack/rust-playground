trait TraitPoint {
    fn new(x: f64, y: f64) -> Self;
    fn x(&self) -> &f64;
    fn y(&self) -> &f64;
    fn distance_to(&self, other: &Self) -> f64 {
        ((*other.x() - *self.x()).powi(2) + (*other.y() - *self.y()).powi(2)).sqrt()
    }
}

struct Point {
    x: f64,
    y: f64
}

impl TraitPoint for Point {
    fn new(x: f64, y: f64) -> Self {
         Point { x: x, y: y }
    }
    fn x(&self) -> &f64 { &self.x }
    fn y(&self) -> &f64 { &self.y }
}

trait TraitRectangle {
    type Point: TraitPoint;

    fn new(topleft: &Self::Point, bottomright: &Self::Point) -> Self;
    fn topleft(&self) -> &Self::Point;
    fn bottomright(&self) -> &Self::Point;

    fn height(&self) -> f64 {
        (*self.topleft().y() - *self.bottomright().y()).abs()
    }

    fn width(&self) -> f64 {
        (*self.topleft().x() - *self.bottomright().x()).abs()
    }

    fn area(&self) -> f64 {
        (self.height() * self.width())
    }
}

struct Rectangle<'a> {
    tl: &'a Point,
    br: &'a Point
}

impl TraitRectangle for Rectangle {
    type Point = Point;

    fn new(topleft: &Point, bottomright: &Point) -> Rectangle {
        Rectangle { tl: topleft, br: bottomright }
    }

    fn topleft(&self) -> &Point {
        self.tl
    }

    fn bottomright(&self) -> &Point {
        self.br
    }
}

fn main() {
    let tl_p = Point { x: 1.1, y: 2.2};
    let br_p = Point { x: 3.3, y: 4.4 };
    let rect = Rectangle { tl: &tl_p, br: &br_p };
    println!("Width: {}", rect.width());
    println!("Height: {}", rect.height());
    println!("AREA: {}", rect.area());
}

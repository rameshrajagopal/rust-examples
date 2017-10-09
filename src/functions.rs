struct Point {
    x: f64,
    y: f64,
}

impl Point {
    #[warn(dead_code)]
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0, }
    }

    fn new(x: f64, y: f64) -> Point {
        Point {x: x, y: y,}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point
}

impl Rectangle {

    fn new(p1: Point, p2: Point) -> Rectangle {
        Rectangle {p1: p1, p2: p2 }
    }

    fn area(&self) -> f64 {
        let Point {x: x1, y: y1 } = self.p1;
        let Point {x: x2, y: y2 } = self.p2;

        ((x1-x2) * (y1-y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1 } = self.p1;
        let Point {x: x2, y: y2 } = self.p2;

        2.0 * ((x1-x2).abs() + (y1-y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;
        self.p2.x += x;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn new(i: i32, j: i32) -> Pair {
        Pair(Box::new(i), Box::new(j))
    }

    fn destroy(self) {
        let Pair(first, second) = self;

        println!("destroy first={} second={}", first, second);
    }
}

fn main() {
    let p1 = Point::new(2.0, 2.0);
    let p2 = Point::new(10.0, 12.0);

    let mut rect = Rectangle::new(p1, p2);

    println!("area={}, peri={}", rect.area(), rect.perimeter());
    rect.translate(3.0, 4.0);
    println!("area={}, peri={}", rect.area(), rect.perimeter());

    let pair = Pair::new(1, 2);
    pair.destroy()
}

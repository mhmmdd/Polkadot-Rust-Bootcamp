// Trait example with Box<T> pointer
struct Point {
    x: i32,
    y: i32,
}

trait PointTrait {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
}

impl PointTrait for Point {
    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }
}


fn main() {
    let point = Box::new(Point { x: 10, y: 20 });
    println!("x = {}, y = {}", point.get_x(), point.get_y());
}

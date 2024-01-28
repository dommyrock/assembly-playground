use print::{print_array, print_bytes};
use std::rc::Rc;
mod print;

struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    stack();
    heap();
    ref_count();
}

#[inline(never)] //we want embeded functions for cargo-asm
pub fn stack() {
    let mut point = Point::new(5, 10);
    point.x += 1;
    point.y += 1;
    print_bytes(&point);
}
#[inline(never)]
pub fn heap() {
    let mut point = Box::new(Point::new(5, 10));
    point.x += 1;
    point.y += 1;
    print_bytes(&point);
}
#[inline(never)]
///Note: Comment println if generating asm here because it adds alot of overhead
pub fn ref_count() {
    let point = Point::new(5, 10);
    let rc = Rc::new(point);

    println!("ref.count before = {}", Rc::strong_count(&rc));

    let second: Rc<Point> = rc.clone();
    println!("ref.count after = {}", Rc::strong_count(&rc));

    println!("\nBellow we see that both var point to the same momo address");
    println!("\t{:p}\n\t{:p}", rc.as_ref(), second.as_ref());
}

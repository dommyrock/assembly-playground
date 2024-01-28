use print::{print_bytes, print_array};
mod print;
struct Point { 
    x: i32,
    y: i32
}
impl Point{ 
    fn new(x:i32,y:i32) -> Self { 
        Self { 
            x,
            y
        }
    }
}

fn main() {
    stack();
    heap();
}
#[inline(never)] //we want embeded functions for cargo-asm
pub fn stack() { 
    let mut point = Point::new(5, 10);
    point.x += 1 ; 
    point.y += 1 ; 
    print_bytes(&point);
}
#[inline(never)]
pub fn heap() { 
    let mut point = Box::new(Point::new(5, 10));
    point.x += 1 ; 
    point.y += 1 ; 
    print_bytes(&point);
}
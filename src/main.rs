use print::{print_bytes, print_array};
mod print;
fn main() {
    test();
}
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

#[inline(never)]
pub fn test() { 
    let mut point = Point::new(5, 10);

    point.x += 1 ; 
    point.y += 1 ; 
    print_bytes(&point);
}
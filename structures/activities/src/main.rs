#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}
#[derive(Debug, Copy, Clone)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom
    // right corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// Function rect_area to calculate area of a Rectangle
// text recommends nested destructuring. lol

//fn rect_area(rect: Rectangle) -> f32 {
//
//   let Rectangle { 
//    top_left: Point { x: topleftpoint.x, y: topleftpoint.y } = rect.p1,
//    bottom_right: Point: { x: bottomrightpoint.x, y: bottomrightpoint.y } = rect.p2 };
//
//    (topleftpoint.x - bottomrightpoint.x) * (topleftpoint.y - bottomrightpoint.y)
//}

fn rect_area(rect: Rectangle) -> f32 {
    let Point { x: x1, y: y1 } = rect.bottom_right;
    let Point { x: x2, y: y2 } = rect.top_left;
    (x1-x2)*(y1-y2)
}

fn square(point: Point, squarep2: f32) -> Rectangle { // returns Rectangle
// with its top left corner on Point and w/h corresponding to float32
    let Point { x: x1, y: y1} = point;
    let square = Rectangle {
        top_left: point,
        bottom_right: Point { x: point.x+squarep2, y: point.y+squarep2 } 
    };
    square
}
fn main() {
    // instantiate a `Point`
    let p1: Point = Point { x: 0.0, y: 5.0 };

    // instantiate a second point valid for the top left 
    // corner of a rectangle
    let p2: Point = Point { x: 5.0, y: 0.0 };

    let _rectangle = Rectangle {
        bottom_right: p1,
        top_left: p2,
    };
    let sqf32: f32 = 5.0;

    println!("The top left point is: ({}, {})", p1.x, p1.y);
    println!("The bottom right point is: ({}, {})", p2.x, p2.y);
    println!("The area of rect is: {:?}", rect_area(_rectangle));
    println!("The coordinates of the top left and bottom right points of
    the generated square are:\n {:?} ", square(p1, sqf32));
}

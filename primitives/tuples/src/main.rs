// The following struct is for the activity.
#[derive(Debug)]
use std::fmt;

// TODO vvv implement
// fmt::Display referring to https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display.html
// which refers to activity 1 from https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html

impl fmt::Display for struct Matrix(f32, f32, f32, f32);

fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);
    
    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    // I couldn't figure out how to print a specific index of the tuple within tuples
    //println!("index 1 of index 2 within tuple_of_tuples is {}", tuple_of_tuples.0);
    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    
    // But long Tuples (more than 12 elements) cannot
    // be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 
    // 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the
    // compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the command is required to
    // tell them from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));
    
    //tuples can be destructured to create bindings
    let tuple = (1, "heloo", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    //Tuples can be used as function arguments and
    // as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to
    // variables
    let (integer, boolean) = pair;

    (boolean, integer)
    }
fn transpose(Matrix: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    let (
}

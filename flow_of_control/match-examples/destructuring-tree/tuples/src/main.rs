// Tuples can be destructured in a math as follows:
fn main() {
    let triple = (3, -2, 4);
    // TODO^ Try different values for `triple`

    println!("Tell me about {:?}", triple);
    //Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?} and `z` is {:?}", y, z),
        (1, ..) => println!("first is `1` and the rest doesn't matter"),
        (.., 2) => println!("last is `2` and the rest doesn't matter"),
        (3, x, 4) => println!("First is `3`, `x` is {:?}, last is `4`", x),
        // `..` can be used to ignore the rest of the tuple
        _ => println!("It doesn't matter what they are"),
    }
}

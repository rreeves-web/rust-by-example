// `get_or_insert() evaluates eagerly, modifies empty value in place
// To make sure that an `Option` contains a value, we can use `get_or_insert` to
// modify it in place with a fallback value, as is shown in the following example.
// Note that `get_or_insert` eagerly evaluates its parameter, so variable `apple`
// is moved:
#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("my_fruit is: {:?}", first_available_fruit);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    // my_fruit is: apple
    // first_available_fruit is: Apple
    //println!("Variable named `apple` is moved: {:?}", apple);
    // TODO: uncomment the line above to see the compiler error
}

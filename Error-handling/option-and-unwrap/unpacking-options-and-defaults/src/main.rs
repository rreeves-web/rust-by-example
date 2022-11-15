// There is more than one way to unpack an `Option` and fall back on a default if
// it is `None`. To choose the one that meets our needs, we need to consider the
// following:
// - do we need eager or lazy evaluation?
// - do we need to keep the original empty value intact, or modify it in place?
fn main() {
    println!("Hello, world!");
}

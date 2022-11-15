// Rust has a few reserved lifetime names. One of those is `'static`. You might
// encounter it in two situations:

// A reference with `'static` lifetime:
let s: &'static str = "hellow world";

// `'static` as part of a trait bound:
fn generic<T>(x: T) where T: 'static {}

// Both are related but subtly different and this is a common source for confusion when
// learning Rust. See main.rs and other files for examples of each situation.

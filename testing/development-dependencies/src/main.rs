// Sometimes there is a need to have dependencies for tests (or examples, or
// benchmarks) only. Such dependencies are added to `Cargo.toml` in the `[dev-
// dependencies]` section. These dependencies are not propagated to other
// packages which depend on this package.

// One such example is `pretty_assertions`, which extends standard `assert_eq!`
// and `assert_ne!` macros, to provide colorful diff.
// Link: https://docs.rs/pretty_assertions/1.0.0/pretty_assertions/index.html
// View fs structure at: https://doc.rust-lang.org/stable/rust-by-example/testing/dev_dependencies.html
fn main() {
    println!("Hello, world!");
}
// See Also
// Cargo docs on specifying dependencies
// Link: http://doc.crates.io/specifying-dependencies.html

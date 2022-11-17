As an introduction to this section, to borrow from the official docs, "one should
try to minimize the amount of unsafe code in a code base." With that in mind,
let's get started! Unsafe annotations in Rust are used to bypass protections put
in place by the compiler; specifically, there are four primary things that unsafe is
used for:
- dereferencing raw pointers
- calling functions or methods which are `unsafe` (including calling a function
    over FFI, see a previous chapter[url]https://doc.rust-lang.org/stable/rust-by-example/std_misc/ffi.html[/url] of the book
- accessing or modifying static mutable variables
- implementing unsafe traits


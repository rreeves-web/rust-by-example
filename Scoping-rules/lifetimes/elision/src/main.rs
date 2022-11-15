// Some lifetime parameters are overwhelmingly common and so the borrow checker
// will allow you to omit them to save typing and to improve readability. This is
// known as elision. Elision exists in Rust solely because these patterns are common.

// The following code shows a few examples of elision. For a more comprehensive
// description of elision, see lifetime elision[url]https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision[/url]
// in the book.

// `elided_input` and `annotated_input` essentially have identical signatures
// because the lifetime of `elided_input` is inferred by the compiler.
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

// Similarly, `elided_pass` and `annotated_pass` have identical signatures
// because the lifetime is added implicitly to `elided_pass`:
fn elided_pass(x: &i32) -> &i32 { x }

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

fn main() {
    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}

// Now inserting an instruction that does nothing is rather boring. Let us do
// something that actually acts on data:

use std::arch::asm;

let x: u64;
unsafe {
    asm!("mov {}, 5", out(reg) x);
}
assert_eq!(x, 5);

// This will write the value `5` into the `u64` variable `x`. You can see that
// the string literal we use to specify instructions is actually a template string.
// It is governed by the same rules as Rust `format strings`[url]https://doc.rust-lang.org/std/fmt/#syntax[/url].
// The arguments that are inserted into the template however look a bit different
// than you may be familiar with. First we need to specify if the variable is an
// input or an output of the inline assembly. In this case it is an output.
// We declared this by writing `out`. We also need to specify in what kind of
// register the assembly expects the variable. In this case we put it in an
// arbitrary general purpose register by specifying `reg`. The compiler will
// choose an appropriate register to insert into the template and will read the
// variable from there after the inline assembly finishes executing.

// Let us see another example that also uses an input:

use std::arch::asm;

let i: u64 = 3;
let o: u64;
unsafe {
    asm!(
        "mov {0}, {1}",
        "add {0}, 5",
        out(reg) o,
        in(reg) i,
    );
}
assert_eq!(o, 8);

// This will add `5` to the input in variable `i` and write the result to variable
// `o`. The particular way this assumbly does this is first copying the value
// from `i` to the output, and then adding `5` to it.

// The example shows a few things:

// First, we can see that `asm!` allows multiple template string arguments; each
// one is treated as a separate line of assembly code, as if they were all joined
// together with newlines between them. This makes it easy to format assembly
// code.

// Second, we can see that inputs are declared by writing `in` instead of `out`.

// Third, we can see that we can specify an argument number, or name as in any
// format string. For inline assembly templates this is particularly useful as
// arguments are often used more than once. For more complex inline assmebly
// using this facility is generally recommended, as it improves readability, and
// allows reordering instructions without changing the argument order.

// We can further refine the above example to avoid the `mov` instruction:

use std::arch::asm;

let mut x: u64 = 3;
unsafe {
    asm!("add {0}, 5", inout(reg) x);
}
assert_eq!(x, 8);

// We can see that `inout` is used to specify an argument that is both input and output.
// This is different from specifying an input and output separately in that it is
// guaranteed to assign both to the same register.

// It is also possible to specify different variables for the input and output
// parts of an `inout` operand:

use std::arch::asm;

let x: u64 = 3;
let y: u64;
unsafe {
    asm!("add {0}, 5", inout(reg) x => y);
}
assert_eq!(y, 8);

// The Rust compiler is conservative with its allocation of operands. It is
// assumed that an `out` can be written at any time, and can therefore not share
// its location with any other argument. However, to guarantee optimal 
// performance it is important to use as few registers as possible, so they won't
// have to be saved and reloaded around the inline assembly block. To achieve
// this Rust provides a `lateout` specifier. This can be used on any output that
// is written only after all inputs have been consumed. There is also a `inlateout`
// variant of this specifier.

// Here is an example where `inlateout` *cannot* be used in `release` mode or
// other optimized cases:

use std::arch::asm;

let mut a: u64 = 4;
let b: u64 = 4;
let c: u64 = 4;
unsafe {
    asm!(
        "add {0}, {1}",
        "add {0}, {2}",
        inout(reg) a,
        in(reg) b,
        in(reg) c,
    );
}
assert_eq!(a, 12);
// The above could work well in unoptimized cases (`Debug` mode), but if you want
// optimized performance (`release` mode or other optimized cases), it could not
// work.

// That is because in optimized cases, the compiler is free to allocate the same
// register for inputs `b` and `c` since it knows they have the same value. However
// it must allocate a separate register for `a` since it uses `inout` and not
// `inlateout`. If `inlateout` was used, then `a` and `c` could be allocated to the
// same register, in which case the first instruction to overwrite the value of `c`
// and case the assembly code to produce the wrong result.

// However the following example can use `inlateout` since the output is only 
// modified after all input registers have been read:

use std::arch::asm;

let mut a: u64 = 4;
let b: u64 = 4;
unsafe {
    asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);
}
assert_eq!(a, 8);

// As you can see, this assembly fragment will still work correctly if `a` and `b`
// are assigned to the same register.

// Let us start with the simplest possible example:

use std::arch::asm;

unsafe {
    asm!("nop");
}

// This will insert a NOP (no operation) instruction into the assmebly generated
// by the compiler. Note that all `asm!` instructions have to be inside an
// `unsafe` block, as they could insert arbitrary instructions and break various
// invariants. The instructions to be inserted are listed in the first argument
// of the `asm!` macro as a string literal.

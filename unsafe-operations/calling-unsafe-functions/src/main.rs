// Some functions can be declared as `unsafe`, meaning it is the programmer's
// responsibility to ensure correctness instead of the compiler's. One example of
// this is `std::slice::from_raw_parts`[url]https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html[/url]
// which will create a slice given a pointer to the first element and a length.

use std::slice;

fn main() {
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
// For `slice::from_raw_parts`, one of the assumptions which *must* be upheld is
// that the pointer passed in points to valid memory and that the memory pointed
// to is of the correct type. If these invariants aren't upheld then the program's
// behavior is undefined and there is no knowing what will happen.

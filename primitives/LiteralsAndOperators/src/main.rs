fn main() {
    // Integer addition
    println!("1+2={}", 1u32+2);

    // Integer subtraction
    println!("1-2={}",1i32-2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type matters

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // I skipped the next four lines because I don't know binary, 
    // octal, or hexidecmical, but I can make sense of it in
    // small doses given that I have the time to think though
    // each number. That stuff should come with time and practice- RR

    // Use underscores to improve readability! // this seems very useful
    println!("One million is written as {}", 1_000_000u32);
}

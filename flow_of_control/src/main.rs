fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three!");

            // Skip the rest of this iteration
            continue;
        }

        if count == 5 {
            println!("Ok, that's enough");

            // Exit this loop
            break;
        }
    }
}

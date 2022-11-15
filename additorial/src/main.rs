fn main() {
let x = 10u32;
add(x);
fn add (mut y: u32) -> () {
    let mut running_total = 0;
        while y > 0 {
            println!("Pullups remaining: {}", y);
            running_total = running_total + y;
            println!("Running total is {}", running_total);
            y = y - 1;
        }
    println!("Pullups complete!");
}
}

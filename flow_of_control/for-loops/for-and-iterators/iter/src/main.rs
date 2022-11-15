// iter borrows each element of the collection through each iteration.
// Thus leaving the collection untouched and available for reuse after
// the loop.
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for selected_name in names.iter() {
        match selected_name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", selected_name),
        }
    }
    println!("names: {:?}", names);
}

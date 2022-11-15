// The previous section illustrates the error handling mechanism `panic`.
// Different code paths can be conditionally compiled based on the panic setting.
// The current values available are `unwind` and `abort`.

// Building on the prior lemonade example, we explicitly use the panic strategy to
// exercise different lines of code.

fn drink(beverage: &str) {
    // You shouldn't drink too many sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic="abort"){ println!("This is not your party. Run!!!");}
        else{ println!("Spit it out!!!");}
    }
    else{ println!("Some refreshing {} is all I need.", beverage);
}}

fn main() {
    drink("water");
    drink("lemonade");
}

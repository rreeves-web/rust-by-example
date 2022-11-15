// If your function is generic over a trait but you don't mind the specific type,
// you can simplify the function declaration using `impl Trait` as the type of the
// argument.

// For example, consider the following code:

fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                    ..collect() // Collect all strings in a row into a Vec<String>
            }}
        }}
        .collect() // Collect all lines into a Vec<Vec<String>>
}

// `parse_csv_document` is generic, allowing it to take any type which implements
// BufRead, such as BufReader<File> or [u8], but it's not important what type `R`
// is, and `R` is only used to declare the type of `src`, so the function can
// also be written as:

fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                    .collect() // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}
// Note that using `impl Trait` as an argument type means that you cannot
// explicitly state what form of the function you use, i.e.
// `parse_csv_document::<std::io::Empty>(std::io::empty())` will not work with
// the second example

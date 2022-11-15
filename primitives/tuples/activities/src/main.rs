use std::fmt; // Import 'fmt'

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Implement 'Display' for 'Matrix'.
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Use 'self.n' to refer to each positional data point.
    write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}
fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Debug: {:?}", matrix);
    println!("Display: {}", matrix);

    println!("Transposed: {}", transpose(matrix));

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    
}

// example function 'reverse' 
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 'let' can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

// activity function 'transpose'
fn transpose(borrowed_matrix: Matrix) -> Matrix {
    let transposed_matrix: Matrix = Matrix(borrowed_matrix.3, borrowed_matrix.2, borrowed_matrix.1, borrowed_matrix.0);
    transposed_matrix
}

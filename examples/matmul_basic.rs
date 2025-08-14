use tensor_algebra_in_rust::matrix;
fn main() {
    // A: 2x3, B: 3x2, C = A * B: 2x2
    let a = matrix![1, 2, 3; 4, 5, 6];
    let b = matrix![7, 8; 9, 10; 11, 12];

    let c = (&a * &b).unwrap();

    println!("Matrix A\n{}", a);
    println!("Matrix B\n{}", b);
    println!("A * B\n{}", c);
}

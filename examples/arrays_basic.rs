use tensor_algebra_in_rust::{matrix, vector};

fn main() {
    let a = vector![1, 2, 3, 4];
    let b = vector![5, 6, 7, 8];

    let sum = a.clone() + b.clone();
    let prod = a.clone() * b.clone();

    let scaled = a.scalar_mul(3);
    let shifted = a.scalar_add(10);

    let dot = a.dot(&b);

    println!("vector a = {}", a);
    println!("vector b = {}", b);
    println!("a + b   = {}", sum);
    println!("a * b   = {}", prod);
    println!("3 * a   = {}", scaled);
    println!("a + 10  = {}", shifted);
    println!("a · b   = {}", dot);
}

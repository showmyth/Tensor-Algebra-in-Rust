use tensor_algebra_in_rust::{matrix, vector};

fn main() {
    let a = matrix![1, 2; 3, 4]; // 2x2
    let b = matrix![5, 6; 7, 8]; // 2x2

    let sum = (a.clone() + b.clone()).unwrap();

    let scaled = a.scalar_mul(2);

    let v = vector![1, -1];
    let mv = a.mat_vec_mul(&v).unwrap();

    println!("Matrix a\n{}", a);
    println!("Matrix b\n{}", b);
    println!("a + b\n{}", sum);
    println!("2 * a\n{}", scaled);
    println!("a * [1, -1]^T = {:?}", mv);
}

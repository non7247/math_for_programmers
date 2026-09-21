fn add(vectors: &[(f64, f64)]) -> (f64, f64) {
    vectors
        .iter()
        .fold((0.0, 0.0), |acc, &(x, y)| (acc.0 + x, acc.1 + y))
}

fn main() {
    let vectors = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0), (4.0, 8.0)];
    let result = add(&vectors);

    println!("{:?}", result);
}
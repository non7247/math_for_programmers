use std::f64::consts::PI;

fn add(vectors: &[(f64, f64)]) -> (f64, f64) {
    vectors
        .iter()
        .fold((0.0, 0.0), |acc, &(x, y)| (acc.0 + x, acc.1 + y))
}

fn length(v: &(f64, f64)) -> f64 {
    (v.0 * v.0 + v.1 * v.1).sqrt()
}

fn subtract(v1: &(f64, f64), v2: &(f64, f64)) -> (f64, f64) {
    (v1.0 - v2.0, v1.1 - v2.1)
}

fn distance(v1: &(f64, f64), v2: &(f64, f64)) -> f64 {
    length(&subtract(v1, v2))
}

fn perimeter(vectors: &[(f64, f64)]) -> f64 {
    let result: f64 = vectors
        .iter()
        .zip(vectors.iter().skip(1))
        .map(|(v1, v2)| distance(v1, v2))
        .sum();
    result + distance(&vectors[0], &vectors[vectors.len() - 1])
}

fn to_cartesian(polar_vector: &(f64, f64)) -> (f64, f64) {
    let (length, angle) = polar_vector;
    (length * angle.cos(), length * angle.sin())
}

fn to_polar(vector: &(f64, f64)) -> (f64, f64) {
    let (x, y) = vector;
    let angle = y.atan2(*x);
    (length(&vector), angle)
}

fn rotate(angle: f64, vectors: &[(f64, f64)]) -> Vec::<(f64, f64)> {
    let polars: Vec<_> = vectors
        .iter()
        .map(|v| to_polar(v))
        .collect();
    polars
        .iter()
        .map(|v| to_cartesian(&(v.0, v.1 + angle)))
        .collect()
}

fn main() {
    let vectors = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0), (4.0, 8.0)];
    let result = add(&vectors);
    println!("{:?}", result);

    let square = vec![(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)];
    let result = perimeter(&square);
    println!("perimeter: {}", result);

    let dino_vectors = vec![
        (6.0, 4.0), (3.0, 1.0), (1.0, 2.0), (-1.0, 5.0), (-2.0, 5.0), (-3.0, 4.0), (-4.0, 4.0),
        (-5.0, 3.0), (-5.0, 2.0), (-2.0, 2.0), (-5.0, 1.0), (-4.0, 0.0), (-2.0, 1.0), (-1.0, 0.0),
        (0.0, -3.0), (-1.0, -4.0), (1.0, -4.0), (2.0, -3.0), (1.0, -2.0), (3.0, -1.0), (5.0, 1.0)
    ];
    let result = perimeter(&dino_vectors);
    println!("perimeter: {}", result);

    for n in -12..=15 {
        for m in -14..=13 {
            if n < 0 || m < 0 || n <= m { continue; }

            let n = n as f64;
            let m = m as f64;
            if (distance(&(n, m), &(1.0, -1.0)) - 13.0).abs() < 0.0001 {
                println!("({},{})", n, m);
            }
        }
    }

    let angle = 37.0 * PI / 180.0;
    let result = to_cartesian(&(5.0, angle));
    println!("({}, {})", result.0, result.1);

    let polar = to_polar(&(1.0, 0.0));
    println!("({}, {})", polar.0, polar.1);
    let polar = to_polar(&(-2.0, 3.0));
    println!("({}, {})", polar.0, polar.1);
}
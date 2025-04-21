fn swap(x: i64, y: i64) -> (i64, i64) {
    return (x, y);
}

fn main() {
    let result = swap(43, 37);
    println!("{}", "{}", result.1, result.0);

    let(a, k) = swap(result.1, result.0);
    println!("{}", "{}", a, k);
}
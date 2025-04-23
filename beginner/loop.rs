fn main() {
    let mut y = 0;
    loop {
        y += 1;
        if y == 90 {
            break;
        }
    }
    println!("{}", y);
}
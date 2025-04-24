fn main() {
    let num d = 0;
    let y = loop{
        d += 1;
        if d == 12 {
            break "12を発見";
        }
    };
    println!("loopの戻り値: {}", y);
}
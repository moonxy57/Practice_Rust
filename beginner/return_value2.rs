fn make_nothing() -> (){
    return ();
}
fn make_nothing2() {
}

fn main(){
    let a = make_nothing;
    let b = make_nothing2;

    println!("a の値: {:?}, a")
    println!("b の値: {:?}, b")
}
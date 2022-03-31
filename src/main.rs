//print hello world in rust
fn main() {
    // start a int with value 10
    let mut x = 10;
    println!("x = {}", x);

    x = 20;
    println!("x = {}", x);
    // end of immutable variable commit

    // start a float with value 10.5

    let y: f64 = 10.5;
    let sum = x + y;
    println!("sum = {}", sum);
}
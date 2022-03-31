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
    let sum = f64::from(x) + y;
    println!("sum = {}", sum);
    // end of adding int to float variable commit

    //start an mutable unsigned int with value 0 and then decrement it
    let mut z: u32 = 0;
    z = z + 1;
    println!("z = {}", z);
    //println!("This line will not be printed 😱");
    // end of runtime panic commit

    // scope and shadowing crashing example
    let x = 5;

    {
        let x = x + 1;
        println!("x = {}", x);
    }
    println!("x = {}", x);
    // end of scope and shadowing commit

    // closure example
    let x = 5;
    let add_ints = |y: i32| -> i32 { y + x };
    println!("add_one = {}", add_ints(5));
    {
        let x = 95;
        println!("x = {}", x);
        println!("add_one = {}", add_ints(5));
    }
    // end of closure commit

    // example of ownership
    let s = String::from("hello");
    let z = s;
    println!("z = {}", z);
    println!("s = {}", s);
}
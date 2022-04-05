//print hello world in rust

use std::thread;
use std::time;

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
    let z = &s;
    println!("z = {}", z);
    println!("s = {}", s);
    // end of ownership commit

    // example of for iterator
    let mut v = vec![1, 2, 3];
    let v_size = 3; // v.len() is also valid, but would not crash
    for i in 0..v_size {
        println!("v[{}] = {}", i, v[i]);
    }
    v = vec![1, 2];
    for i in 0..v_size - 1{
        println!("v[{}] = {}", i, v[i]); // this will crash at index 2 (third element)
    }

    println!("-----");
    let vector = vec![5, 6, 7, 8, 9, 10];
    for element in vector {
        println!("vector[?] = {}", element);
    }
    println!("-----");
    let vector = vec![5, 6, 7, 8, 9, 10];
    for (position, element) in vector.iter().enumerate() {
        println!("v[{}] = {}", position, element);
    }
    // end of for iterator commit

    // example of function
    let mut x:i32 = 5;
    add_one_and_print(&mut x);   // prints 6
    println!("x = {}", x);  // prints 6
    x += 1;
    println!("x = {}", x); // prints 7, no moved value error
    // end of function commit, but is it thread safe?

    // example of thread:
    // starts x=0
    // spawns a thread that increments x by 1 using add_one_and_print
    // at main thread, increments x by 1 using add_one_and_print
    // joins the thread
    println!("-----");
    let mut x:i32 = 0;
    println!("x = {}", x);
    let thread_handle = thread::spawn( || {
        for _ in 0..100 {
            add_one_and_print(&mut x);
        }
    });
    for _ in 0..100 {
        add_one_and_print(&mut x);
    }
    thread_handle.join().unwrap();
}

fn add_one_and_print(x: &mut i32) {
    *x = *x + 1;
    println!("x = {}", x);
}
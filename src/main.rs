 fn main() {
    let x: i32 = 1;
    println!("Hello, world!");
    // BY default everything is immutable in Rust
    let mut result: i32 = sum(127, 1);
    println!("The value of x is: {}", x);
    let z: f32 = 2.0;
    println!("The value of result is: {}", result);
    println!("The value of z is: {}", z);
    println!("The sum of 127 and 1 is: {}", result);

    let greeting: String = String::from("Hello, Rust!");
    println!("{}", greeting);

    let isEven: bool = true;
    if isEven {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }

    for i in 1..5 {
        println!("Iteration number: {}", i);
    }

    // Stack vs Heap in Rust
    /*
        stack is used when we have fixed size data known at compile time, while heap is used for       dynamic size data for examples strings and vectors.
     */

    stack_fun();
    heap_fun();
    update_string(); 


}

fn stack_fun() {
    let a: i32 = 10;
    let b: i32 = 20;
    let c: i32 = a + b;
    println!("Stack function result: {}", c);
}

fn heap_fun() {
    let s1 = String::from("Hello, heap!");
    let s2 = s1 + " Welcome to Rust.";
    println!("Heap function result: {}", s2);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);
    println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    // Append some text to the string
    for _ in 0..100{
        s.push_str(" and some additional text");
        // println!("After update: {}", s);
        println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    }
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
} 
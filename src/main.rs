 fn main() {
    let x: i32 = 1;
    println!("Hello, world!");
    // BY default everything is immutable in Rust
    let result: i32 = sum(127, 1);
    println!("The value of x is: {}", x);
    let z: f32 = 2.0;
    println!("The value of result is: {}", result);
    println!("The value of z is: {}", z);
    println!("The sum of 127 and 1 is: {}", result);

    let greeting: String = String::from("Hello, Rust!");
    println!("{}", greeting);

    let is_even: bool = true;
    if is_even {
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

    // Ownership in Rustb 
    /*
    Set of rules that govern how a Rust program manages memory. Ownership is a key concept in Rust that ensures memory safety without needing a garbage collector.

    1. Each value in Rust has a variable that’s called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.


     */
    let s1 = String::from("Ownership in Rust");
    let _s2 = s1; // s1 is moved to s2, s1 is no longer valid as it might create dangling pointer/double free error issues in future
    // println!("{}", s1); // This would cause a compile-time error because s2 ke paas ownership hai ab

    let mut my_string = String::from("Hello");
    my_string = some_fun(my_string);
    println!("{}", my_string);

    // Referencing
    let s3 = String::from("Hello, references!");
    take_ownership(&s3);
    println!("{}", s3); // This is valid because we passed a reference and ownership was not moved

    // multiple borrowers allowed, unless no hanky panky
    let s4 = &s3;
    let s5 = &s3;
    let s6 = &s3;
    let s7 = &s3;
    let s8 = &s3;

    println!("Multiple references: {}, {}, {}, {}, {}", s4, s5, s6, s7, s8);

}

fn take_ownership(some_string: &String) {
    println!("Inside take_ownership: {}", some_string);
} // some_string goes out of scope here and is dropped

fn some_fun(some_string: String) -> String {
    println!("Inside some_fun: {}", some_string);
    return some_string;
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
    for _ in 0..5{
        s.push_str(" and some additional text");
        // println!("After update: {}", s);
        // You can increase the for loop count to see how pointer location changes after some point
        println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    }
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
} 
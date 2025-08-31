 fn main() {
    let x = 1;
    println!("The value of x is: {}", x);
    println!("Hello, world!");
    let mut result: i32 = sum(127, 1);
    let z: f32 = 2.0;
    println!("The value of z is: {}", z);
    println!("The sum of 127 and 1 is: {}", result);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
} 
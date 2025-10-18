 fn main() {
    let x: i32 = 1;
    println!("Hello, world!");
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

    // Iteration of arrays, maps, strings


}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
    return &s[..];
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
} 
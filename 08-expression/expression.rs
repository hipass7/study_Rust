fn main() {
    let c = add(1, 2);
    println!("{}", c);
}
 
fn add(a: i32, b: i32) -> i32 {
    dbg!(a, b);
    a + b // expression
}

// 1. Concise and clear code
// 2. Immutability and functional programming style
// 3. Error prevention
// 4. Code reuse and abstraction
// 5. Consistency
// 6. Flexible control flow
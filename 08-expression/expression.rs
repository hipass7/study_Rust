fn main() {
    let number = 13;
    
    // 1. Concise and clear code
    let odd = if number % 2 == 0 { "even" } else { "odd" };
    println!("{}: {}", number, odd);

    // 2. Immutability and functional programming style
    let y = {
        let x = 3;
        x + 1
    };

    // 3. Error prevention
    fn add_one(x: i32) -> i32 {
        x + 1  // No semicolon, so this value is returned
    }

    // 4. Code reuse and abstraction
    let c = add(1, 2);
    println!("{}", c);

    // 5. Consistency
    let result = if number % 2 == 0 {
        42
    } else {
        0
    };

    // 6. Flexible control flow
}
 
fn add(a: i32, b: i32) -> i32 {
    dbg!(a, b);
    a + b // expression
}

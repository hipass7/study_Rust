fn main()
{
    let a = 100; // default : i32
    let b = 3.14; // default : f64
    let c: u32 = 12345;
    let d: f32 = 3.14;
    let e: bool = true;

    // Immutability
    let x = 100;
    // x = x + 1; // compile error : cannot assign twice to immutable variable 'x'
    println!("{}", x);

    // mutability
    let mut y = 100;
    y = y + 1;
    println!("{}", y);
}
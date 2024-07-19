fn main() {
    // constant
    const PI: f64 = 3.141592;
 
    let area = PI * 5.0 * 5.0;
    println!("{}", area);

    // shadowing
    let a = 1;
    println!("{}", a);

    let a = "hello";
    println!("{}", a);

    let a = 1;
    let a = 2;
    {
        let a = a + 1;
        println!("{}", a); // 3
    }
    println!("{}", a); // 2

    let mut a = 1;
    a = 2;
    {
        a = a + 1;
        println!("{}", a); // 3
    }
    println!("{}", a); // 3

}
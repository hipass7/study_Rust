fn main()
{   
    // string literal
    let s = "hello"; // let s: &'static str = "hello";
    println!("{}", s);

    let sub: &str = &s[1..4];
    println!("{}", sub); // ell

    // change to String type
    let x: String = s.to_owned();
    println!("{}", x);

    // methods : trim, to_lowercase
    let s: &str = "   HELLO   ";
    println!("{} {}", s.trim(), s.to_lowercase());
}
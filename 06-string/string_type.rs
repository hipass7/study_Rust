fn main()
{
    let s = String::from("hello");
    println!("{}", s);

    let mut s = String::new();
    s.push('H');
    s.push('i');
    s.push_str(" Tom");

    println!("{}", s);

    s = s.replace("Hi", "Hello");
    println!("{}", s);

    if s.contains(" ") {
        for w in s.split_whitespace() {
            println!("{}", w);
        }
    }
}
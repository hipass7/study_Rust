struct Member {
    fname: String,
    lname: String,
    age: u16,
    active: bool
}

fn main() {
    let mut mem2 = Member {
        active: true,
        fname: String::from("Justin"),
        lname: String::from("Kim"),
        age: 35
    };
 
    // modify struct field
    mem2.active = false;
 
    // read struct field 
    println!("{}: {}", mem2.fname, mem2.active);
}
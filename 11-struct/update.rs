struct Admin {
    name: String,
    group: String,
    active: bool
}
 
fn main() {
    let adm1 = Admin {
        name: String::from("Tom"),
        group: String::from("IT"),
        active: true
    };
 
    let adm2 = Admin {
        name: String::from("Kim"),
        ..adm1     // struct update syntax
    };
 
    println!("{}", adm2.group); // OK
    println!("{}", adm1.name);  // OK
    //println!("{}", adm1.group); // compile error because group field is owned by adm2.
}
fn main() {
    let mem = get_member("Tom".to_owned(), "Lee".to_owned(), 33);
    //...
}
 
fn get_member(fname: String, lname: String, age: u16) -> Member {
    Member {
        fname: fname,  // Origin
        lname,         // OK
        age,           // OK
        active: true
    }
}

// If the parameter name transmitted to the function and the structure field name are the same, it may be omitted during structure initialization.
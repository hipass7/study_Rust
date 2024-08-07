fn main() {
    let mut i = 1;
 
    loop {
        if i > 10 {
            break;
        }
 
        println!("{}", i);
        
        i += 1;
    }
}
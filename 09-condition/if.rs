fn main() {
    // example) statement
    let score = 'C';
 
    if score <= 'B' {
        println!("Excellent");            
    }    
    else if score <= 'C' {
        println!("Pass");            
    }
    else {
        println!("Fail");            
    }

    // example) expression
    let score = 'B';
 
    let ok = if score <= 'C' { 
        println!("{}", score);
        "Pass"
    } 
    else { 
        "Fail"
    };
     
    println!("{}", ok);
}
#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High
}

// data can be assigned to each type.
#[derive(Debug)]
enum LogType {
    Info(String),      //(infoMsg)
    Warning(String),   //(warningMsg)
    Error(i32, String) //(errorCode, errorMsg)
}
 
fn main() {
    let urgent = Priority::High;
    dbg!(urgent);

    let log = LogType::Error(1201, String::from("Not found"));
    println!("{:?}", log);


}
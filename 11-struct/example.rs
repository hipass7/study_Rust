struct App {
    startTime: u16,
    endTime: u16,
    name: String,
    active: bool
}

struct Bank;

fn main() {
    let mut kakaoMap = App {
        active: true,
        startTime: 30,
        endTime: 100,
        name: String::from("kakaoMap"),
    };

    let wooriBank = Bank;

    kakaoMap.startTime = 50;

    println!("{}: {}", kakaoMap.name, kakaoMap.startTime);
}
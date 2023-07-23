// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move(i32, i32),
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("hello world")));
    println!("{:?}", Message::Move(100, 200));
    println!("{:?}", Message::ChangeColor);
}

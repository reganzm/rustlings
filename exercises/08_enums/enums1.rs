#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::Echo(String::from("Hello, world!")));
    println!("{:?}", Message::ChangeColor(255, 0, 0));
    println!("{:?}", Message::Quit);
}

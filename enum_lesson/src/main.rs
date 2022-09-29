fn main() {
    let m = Message::Write(String::from("HOGE"));
    m.call();

    let q = Message::Quit;
    q.call();

    let mv = Message::Move { x: 100, y: 200 };
    mv.call();

    let cc = Message::ChangeColor(123, 456, 789);
    cc.call();
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move to x: {}, y: {}", x, y),
            Message::Write(message) => println!("a sent message is: {}", message),
            Message::ChangeColor(r, g, b) => println!("change color to: {}, {}, {}", r, g, b),
        }
    }
}

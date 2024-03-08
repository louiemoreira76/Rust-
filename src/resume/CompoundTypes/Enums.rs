// C-like enum
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Number2 {
    Zero = 0,
    One = 1,
    Two = 5,
    three, //6
}

fn main() {
    let msgs: [Message;3] = [ //primeiro o tipo, dps quanto suporta
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];
    for msg in msgs {
        println!("{:?}", msg)
    }

    let n: Option<i32> = None;
    let five: Option<i32> = Some(5);
    println!("{:?} {:?}", five, n);
} 
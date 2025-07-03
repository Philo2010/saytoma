use iced::widget::{button, column, text, Column};

#[test]
fn it_counts_properly() {
    let mut counter = Counter { count: 0 };

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.count, 1);
}

#[derive(Default)]
struct Counter {
    count: u64
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Decrement => {self.count -=1;}
            Message::Increment => {self.count +=1;}
        }
    }
    fn view(&self) -> Column<Message>  {
        column![
            button("+").on_press(Message::Increment),
            text(self.count),
            button("-").on_press(Message::Decrement),
        ]
    }
}


//pub fn main() -> iced::Result {
//    iced::run("A cool program", Counter::update, Counter::view)
//}
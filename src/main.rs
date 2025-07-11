use std::{fs::File, path::{Path, PathBuf}};
use iced::{
    Application, Element, Settings, executor,
    widget::{Image, button, column, text, Column},
    keyboard, Subscription,
};
mod raw_reader;

pub fn main() -> iced::Result {
    iced::application("A counter", Saytoma::update, Saytoma::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Open(String)
}

struct Saytoma {
    page: usize,
    reader: Option<raw_reader::PageReader>
}

impl Default for Saytoma {
    fn default() -> Self {
        Saytoma { 
            page: 0,
            reader: None
        }
    }
}

impl Saytoma {
    fn open_new_file(&mut self,name: &String) {
        let path = Path::new(name);

        let file = match File::open(path) {
            Ok(a) => a,
            Err(_) => return, //File not found
        };

        //File so exist from this point

        let reader_local = match raw_reader::PageReader::new(file) {
            Ok(a) => a,
            Err(_) => return
        }; 

        self.reader = Some(reader_local);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.page += 1,
            Message::Decrement => self.page -= 1,
            Message::Open(a) => self.open_new_file(&a),
        }
    }

    fn view(&self) -> Column<Message> {
        if self.reader.is_none() {
            return column![text("No file loaded"),
            button("Open").on_press(Message::Open("/Users/philipbedrosian/code/saytoma/One-Punch Man Chapters 101-105.cbz".to_string()))
            ];
        }
        let unwrap_reader = self.reader.as_ref().unwrap();

        column![
            button("+").on_press(Message::Increment),
            button("-").on_press(Message::Decrement),
            Image::new(unwrap_reader.read_at(self.page)),
        ]
    }
}

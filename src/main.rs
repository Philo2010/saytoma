use std::{fs::File, path::{Path, PathBuf}};
use iced::{
    Application, Command, Element, Settings, executor,
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
    KeyPressed(keyboard::Event)
}

struct Saytoma {
    page: u32,
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
            Message::KeyPressed(x) => {
                todo!()
            }
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        keyboard::Events::default().map(Message::KeyPressed)
    }

    fn view(&self) -> Column<Message> {
        if self.reader.is_none() {
            return column!(text("No file loaded"));
        }
        let unwrap_reader = self.reader.as_ref().unwrap();

        column![
            Image::new(unwrap_reader.paths[0].clone()),
            button("+").on_press(Message::Increment),
        ]
    }
}

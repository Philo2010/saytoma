use std::{fs::File, path::{Path, PathBuf}};
use iced::{
    advanced::{graphics::text::cosmic_text::Scroll, svg::Handle}, executor, keyboard, mouse, widget::{button, canvas, column, image, text, Column, Image, Scrollable}, Application, Color, Element, Length, Rectangle, Renderer, Settings, Subscription, Theme, Vector
};
mod raw_reader;
//TODO:
//1. Add a progress bar while unziping,
//2. Add Keybinds for open, right, and left
//3. Make the ui not shit
pub fn main() -> iced::Result {
    iced::application("A counter", Saytoma::update, Saytoma::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    ZoomIn,
    ZoomOut,
    Open(String)
}

struct Saytoma {
    page: usize,
    reader: Option<raw_reader::PageReader>,
    zoom: f32
}

impl Default for Saytoma {
    fn default() -> Self {
        Saytoma { 
            page: 0,
            reader: None,
            zoom: 1.0
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
            Message::Increment => self.page += 1, //TODO: Clamp this
            Message::Decrement => self.page -= 1,
            Message::Open(a) => self.open_new_file(&a),
            Message::ZoomIn => self.zoom *= 1.1,
            Message::ZoomOut => self.zoom /= 1.1,
        }
    }

    fn view(&self) -> Scrollable<Message> {
        if self.reader.is_none() {
            let collom = column![
            text("No file loaded"),
            button("Open").on_press(Message::Open("/Users/philipbedrosian/Downloads/Invincible, Vol. 2.cbz".to_string()))
            ];
            return Scrollable::new(
            Column::new().push(collom));
        }
        let unwrap_reader = self.reader.as_ref().unwrap();

        let handle = iced::advanced::image::Handle::from_path(unwrap_reader.read_at(self.page));
        let (base_width, base_height) = (800.0, 1000.0); 

        let collom = column![
            button("+").on_press(Message::Increment),
            button("-").on_press(Message::Decrement),
            button("zoom in").on_press(Message::ZoomIn),
            button("zoom out").on_press(Message::ZoomOut),
            iced::widget::image::Image::new(handle)
                .width(Length::from((base_width * self.zoom) as u16))
                .height(Length::from((base_height * self.zoom) as u16))
            //iced::widget::image::Viewer::new(handle).scale_step(self.zoom).height(1000),
            //canvas(ImageView {radius: 50.0})
        ];
        Scrollable::new(
            Column::new().push(collom))
    }
}


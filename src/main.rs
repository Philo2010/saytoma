use std::{fmt::{write, Debug}, fs::File, ops::Sub, path::{Path, PathBuf}, process::Command, sync::{atomic::{AtomicUsize, Ordering}, Arc}, time::Duration};
use iced::{
    futures::{channel::mpsc::Sender, future::Shared}, stream, time, widget::{button, column, progress_bar, text, Column, Scrollable}, Length, Task
};
use tokio::sync::watch;
mod raw_reader;
use iced::Subscription;

//TODO:
//1. Add a progress bar while unziping,
//2. Add Keybinds for open, right, and left
//3. Make the ui not shit
pub fn main() -> iced::Result {
    iced::application("A counter", Saytoma::update, Saytoma::view)
    .subscription(|state | state.subscription())
    .run()
}

enum Message {
    Increment,
    Decrement,
    ZoomIn,
    ZoomOut,
    Open(String),
    Tick,
    DoneLoading(Result<raw_reader::PageReader, std::io::Error>),
}

struct Saytoma {
    page: usize,
    reader: Option<raw_reader::PageReader>,
    zoom: f32,
    counter: usize,
    loading_stream: Option<Arc<AtomicUsize>>,
}

impl Default for Saytoma {
    fn default() -> Self {
        Saytoma { 
            page: 0,
            reader: None,
            zoom: 1.0,
            counter: 0,
            loading_stream: None,
        }
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::Increment => Message::Increment,
            Message::Decrement => Message::Decrement,
            Message::ZoomIn => Message::ZoomIn,
            Message::ZoomOut => Message::ZoomOut,
            Message::Tick => Message::Tick,
            Message::Open(s) => Message::Open(s.clone()),
            Message::DoneLoading(_) => {
                panic!("Cannot clone Message::DoneLoading");
            }
        }
    }
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Increment => write!(f, "Increment"),
            Message::Decrement => write!(f, "Decrement"),
            Message::ZoomIn => write!(f, "ZoomIn"),
            Message::ZoomOut => write!(f, "ZoomOut"),
            Message::Tick => write!(f, "Tick"),
            Message::Open(s) => f.debug_tuple("Open").field(s).finish(),
            Message::DoneLoading(_) => write!(f, "DoneLoading(<opaque>)"),
        }
    }
}



impl Saytoma {
    fn open_new_file(&mut self,name: &String) -> Task<Message> {
        if self.loading_stream.is_some() {
            return Task::none();
        }
        let path = Path::new(name);

        let file = match File::open(path) {
            Ok(a) => a,
            Err(_) => return Task::none(), //File not found
        };

        //File so exist from this point

        let tx = Arc::new(AtomicUsize::new(0));
        self.loading_stream = Some(tx.clone());

        Task::perform(raw_reader::PageReader::new(file, tx), Message::DoneLoading)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => {self.page += 1; Task::none()}, //TODO: Clamp this
            Message::Decrement => {self.page -= 1; Task::none()},
            Message::Open(a) => self.open_new_file(&a),
            Message::ZoomIn => {self.zoom *= 1.1; Task::none()},
            Message::ZoomOut => {self.zoom /= 1.1; Task::none()},
            Message::Tick => {
                self.counter =  match &self.loading_stream {
                    None => 0,
                    Some(a) => a.load(Ordering::SeqCst),
                };
                Task::none()
            }
            Message::DoneLoading(a) => {
                match a {
                    Err(_) => {
                        self.reader = None;
                        self.loading_stream = None;
                        Task::none()
                    }
                    Ok(e) => {
                        self.reader = Some(e);
                        self.loading_stream = None;
                        Task::none()
                    }
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.loading_stream.is_none() {
            return Subscription::none();
        }
        time::every(Duration::from_millis(100)).map(|_| Message::Tick)
    }

    fn view(&self) -> Scrollable<Message> {
        if self.reader.is_none() {
            let collom = column![
            progress_bar(0.0..=100.0, self.counter as f32),
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


use std::{fs::File, ops::Sub, sync::{atomic::{AtomicUsize, Ordering}, Arc}, time::Duration};
use iced::{
    advanced::subscription, alignment::Horizontal, keyboard::{self, Key, Modifiers}, time, widget::{button, center, column, container, progress_bar, scrollable::{self, AbsoluteOffset, RelativeOffset}, text, Column, Container, Scrollable}, Alignment, Length::{self}, Task
};
mod raw_reader;
use iced::Subscription;
use rfd::FileDialog;
use iced::advanced::widget::operation;

pub fn main() -> iced::Result {
    iced::application("A counter", Saytoma::update, Saytoma::view)
    .subscription(|state | state.subscription())
    .run()
}

enum ScrollDir {
    Up,
    Down
}

enum Message {
    Increment,
    Decrement,
    ZoomIn,
    ZoomOut,
    Open,
    Tick,
    ScrollUp,
    ScrollDown,
    ScrollStop,
    DoneLoading(Result<raw_reader::PageReader, std::io::Error>),
    NoInput
}

struct Saytoma {
    page: usize,
    reader: Option<raw_reader::PageReader>,
    zoom: f32,
    counter: usize,
    loading_stream: Option<Arc<AtomicUsize>>,
    scroll: scrollable::Id,
    scroll_y: f32,
    scroll_dir: Option<ScrollDir>,
}

impl Default for Saytoma {
    fn default() -> Self {
        Saytoma { 
            page: 0,
            reader: None,
            zoom: 1.0,
            counter: 0,
            loading_stream: None,
            scroll: scrollable::Id::unique(),
            scroll_y: 0.0,
            scroll_dir: None
        }
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::NoInput => Message::NoInput,
            Message::Increment => Message::Increment,
            Message::Decrement => Message::Decrement,
            Message::ZoomIn => Message::ZoomIn,
            Message::ZoomOut => Message::ZoomOut,
            Message::Tick => Message::Tick,
            Message::Open => Message::Open,
            Message::ScrollUp => Message::ScrollUp,
            Message::ScrollStop => Message::ScrollStop,
            Message::ScrollDown => Message::ScrollDown,
            Message::DoneLoading(_) => {
                panic!("Cannot clone Message::DoneLoading");
            }
        }
    }
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::NoInput => write!(f, "NoInput"),
            Message::Increment => write!(f, "Increment"),
            Message::Decrement => write!(f, "Decrement"),
            Message::ZoomIn => write!(f, "ZoomIn"),
            Message::ZoomOut => write!(f, "ZoomOut"),
            Message::Tick => write!(f, "Tick"),
            Message::Open=> write!(f, "Open"),
            Message::ScrollStop => write!(f, "ScrollStop"),
            Message::ScrollDown => write!(f, "ScrollDown"),
            Message::ScrollUp => write!(f, "ScrollDown"),
            Message::DoneLoading(_) => write!(f, "DoneLoading(<opaque>)"),
        }
    }
}



impl Saytoma {
    fn open_new_file(&mut self, file: File) -> Task<Message> {
        if self.loading_stream.is_some() {
            return Task::none();
        }

        let tx = Arc::new(AtomicUsize::new(0));
        self.loading_stream = Some(tx.clone());
        self.reader = None;

        Task::perform(raw_reader::PageReader::new(file, tx), Message::DoneLoading)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ScrollDown => {
                self.scroll_dir = Some(ScrollDir::Down);
                Task::none()
            },
            Message::ScrollUp => {
                self.scroll_dir = Some(ScrollDir::Up);
                Task::none()
            },
            Message::ScrollStop => {
                self.scroll_dir = None;
                Task::none()
            }
            Message::NoInput => {Task::none()},
            Message::Increment => {
                if let Some(reader) = &self.reader {
                    self.page = (self.page + 1).min(reader.paths.len().saturating_sub(1));
                }
                Task::none()
            },
            Message::Decrement => {
                if self.reader.is_some() {
                    self.page = self.page.saturating_sub(1);
                }
                Task::none()
            },
            Message::Open => {
                let filebuf = match FileDialog::new().pick_file() {
                    None => {return Task::none();},
                    Some(a) => a,
                };
                let file = match File::open(filebuf) {
                    Err(_) => {return Task::none();},
                    Ok(a) => a
                };
                self.open_new_file(file)
            },
            Message::ZoomIn => {self.zoom *= 1.1; Task::none()},
            Message::ZoomOut => {self.zoom /= 1.1; Task::none()},
            Message::Tick => {
                self.counter =  match &self.loading_stream {
                    None => 0,
                    Some(a) => a.load(Ordering::SeqCst),
                };

                if let Some(value) = &self.scroll_dir {
                    match value {
                        ScrollDir::Up => {
                            self.scroll_y = (self.scroll_y - 0.10).max(0.0);
                            return scrollable::snap_to(
                                self.scroll.clone(),
                                RelativeOffset { x: 0.0, y: self.scroll_y },
                            );
                        },
                        ScrollDir::Down => {
                            self.scroll_y = (self.scroll_y + 0.10).min(1.0);
                            return scrollable::snap_to(
                                self.scroll.clone(),
                                RelativeOffset { x: 0.0, y: self.scroll_y }, 
                            );
                        },
                    }
                }
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

    fn keyboard_sub(&self) -> Subscription<Message> {
        iced::event::listen().map(|event| match event {
            iced::Event::Keyboard(a) => {
                match a {
                    keyboard::Event::KeyReleased {key, ..} => {
                        match key {
                            keyboard::Key::Named(keyboard::key::Named::ArrowRight) => {
                                return Message::Increment;
                            }
                            keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => {
                                return Message::Decrement;
                            }
                            keyboard::Key::Character(c) => {
                                if c == "+" || c == "=" {
                                    return Message::ZoomIn;
                                } else if c == "-" || c == "_" {
                                    return Message::ZoomOut;
                                } else if c == "o" || c == "O" {
                                    return Message::Open;
                                } else {
                                    return Message::NoInput;
                                }
                            }
                            keyboard::Key::Named(keyboard::key::Named::ArrowDown) => {
                                return Message::ScrollStop;
                            }
                            keyboard::Key::Named(keyboard::key::Named::ArrowUp) => {
                                return Message::ScrollStop;
                            }
                            _ => {return Message::NoInput;}
                        }
                    },
                    keyboard::Event::KeyPressed {key, ..} => {
                        match key {
                            keyboard::Key::Named(keyboard::key::Named::ArrowDown) => {
                                return Message::ScrollDown;
                            }
                            keyboard::Key::Named(keyboard::key::Named::ArrowUp) => {
                                return Message::ScrollUp;
                            }
                            _ => {return Message::NoInput;}
                        }
                    }
                    _ => {return Message::NoInput;}
                }
            }
            _ => {return Message::NoInput}
        })
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subs = Vec::new();

        subs.push(self.keyboard_sub());

        subs.push(time::every(Duration::from_millis(100)).map(|_| Message::Tick));
        Subscription::batch(subs)
    }

    fn view(&self) -> Container<Message> {
        if self.reader.is_none() {
            if self.loading_stream.is_some() {
                let collom = column![
                    progress_bar(0.0..=100.0, self.counter as f32),
                ];
                return center(collom);
            }
            let collom = column![
                text("Saytoma").size(48),
                text("a simple comic book reader | MIT LICENSE Philip Bedrosian 2025"),
                text("Up | scroll up"),
                text("Down | scroll down"),
                text("Left | Last page"),
                text("Right | Next page"),
                text("+ | Zoom in"),
                text("- | Zoom out"),
                button("Open").on_press(Message::Open)
            ].align_x(Alignment::Center);
            return center(collom);
        }
        let unwrap_reader = self.reader.as_ref().unwrap();

        let handle = iced::advanced::image::Handle::from_path(unwrap_reader.read_at(self.page));
        let (base_width, base_height) = (800.0, 1000.0); 
        let collom = column![
            iced::widget::image::Image::new(handle)
                .width(Length::from((base_width * self.zoom) as u16))
                .height(Length::from((base_height * self.zoom) as u16)),
        ];

        center(Scrollable::new(collom).id(self.scroll.clone()))
    }
}


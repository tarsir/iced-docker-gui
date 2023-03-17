use iced::executor;
use iced::{Application, Command, Element, Theme};

pub struct Playground {
    message: String,
}

impl Application for Playground {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Playground{message: "hello".to_string()}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Playground for Iced")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        self.message.as_str().into()
    }
}

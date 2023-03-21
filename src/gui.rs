use iced::alignment::{self, Alignment};
use iced::widget::{self, container, text, column, row, scrollable};
use iced::executor;
use iced::{Application, Command, Element, Length, Theme};
use crate::docker::{DockerImage, images};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct UiError;

impl Error for UiError {}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something went wrong with the UI")
    }
}

#[derive(Debug)]
pub enum Playground {
    Loading,
    Ready(State)
}

#[derive(Debug, Default, Clone)]
pub struct State {
    image_list: Vec<DockerImage>,
}

#[derive(Debug)]
pub enum Message {
    Loading,
    Ready(Result<State, UiError>)
}

impl Application for Playground {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Playground::Loading, Command::perform(get_images(), Message::Ready))
    }

    fn title(&self) -> String {
        String::from("Playground for Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            Playground::Loading => {
                match message {
                    Message::Ready(Ok(state)) => {
                        *self = Playground::Ready(state.clone())
                    },
                    Message::Ready(Err(_)) => {
                        *self = Playground::Ready(State::default())
                    },
                    _ => {}
                }
                Command::none()
            },
            Playground::Ready(state) => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match self {
            Playground::Loading => loading_message(),
            Playground::Ready(state) => image_list()
        }
    }
}

async fn get_images() -> Result<State, UiError> {
    if let Ok(images) = images() {
        Ok(State { image_list: images })
    } else {
        Err(UiError {})
    }
}

fn image_list<'a>() -> Element<'a, Message> {
    container(
        text("Hi!")
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(40)
    )
    .into()
}

fn loading_message<'a>() -> Element<'a, Message> {
    container(
        text("Loading...")
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .center_x()
    .into()
}

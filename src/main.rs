use iced::{Application, Settings};
mod docker;
mod gui;

fn main() -> iced::Result {
    gui::Playground::run(Settings::default())
}


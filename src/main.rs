use iced::{Application, Settings};
mod docker;
mod gui;

fn main() -> iced::Result {
    println!("{:?}", docker::images().unwrap());
    gui::Playground::run(Settings::default())
}


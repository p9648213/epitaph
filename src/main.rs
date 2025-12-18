use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{column, container, text},
    window,
};

#[derive(Default)]
struct App {}

#[derive(Debug, Clone)]
pub enum Message {}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        container(column![text("Hello World")]).center(Fill).into()
    }

    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }
}

fn init() -> (App, Task<Message>) {
    (
        App::default(),
        window::latest().and_then(|id| window::maximize(id, true)),
    )
}

fn main() -> iced::Result {
    iced::application(init, App::update, App::view)
        .window(window::Settings {
            maximized: true,
            ..Default::default()
        })
        .run()
}

use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{button, column, container},
    window,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenSnake,
}

#[derive(Default)]
struct App {}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        container(column![button("Open Snake").on_press(Message::OpenSnake)]).center(Fill).into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenSnake => {
                open_snake()
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn open_snake() {
    use std::process::Command;

    Command::new("/home/tiso/Desktop/code/tiso-os/games/snake/target/release/snake")
        .spawn()
        .unwrap();
}

#[cfg(target_arch = "wasm32")]
fn open_snake() {
    use wasm_bindgen::JsCast;
    use web_sys::{window, HtmlIFrameElement};

    let window = window().expect("no window");
    let document = window.document().expect("no document");
    let body = document.body().expect("no body");

    // Create iframe
    let iframe = document
        .create_element("iframe")
        .unwrap()
        .dyn_into::<HtmlIFrameElement>()
        .unwrap();

    // Fullscreen styling
    iframe.set_src("snake/index.html");
    iframe.set_width("100%");
    iframe.set_height("100%");
    iframe
        .style()
        .set_property("border", "none")
        .unwrap();
    iframe
        .style()
        .set_property("position", "fixed")
        .unwrap();
    iframe
        .style()
        .set_property("top", "0")
        .unwrap();
    iframe
        .style()
        .set_property("left", "0")
        .unwrap();
    iframe
        .style()
        .set_property("z-index", "9999")
        .unwrap();

    // Attach to page
    body.append_child(&iframe).unwrap();
    iframe.focus().unwrap();
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

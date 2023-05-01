mod notes;
use iced::widget::{column, Text, TextInput};
use iced::{
    keyboard::{
        Event::{CharacterReceived, KeyPressed},
        KeyCode, Modifiers,
    },
    Application, Command, Element,
    Event::Keyboard,
    Settings, Subscription, Theme,
};
use notes::Notes;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug)]
struct App {
    notes: Notes,
    command: (bool, String),
    error: Option<String>,
}
#[derive(Debug, Clone)]
enum AppMessage {
    NewNote(String),
    NewFolder(String),
    DeleteNote(String),
    DeleteFolder(String),
    Open(String),
    Cd(String),
    UpdateCommand(String),
    RunCommand,
    Event(iced_native::Event),
}
impl Application for App {
    type Message = AppMessage;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();
    fn title(&self) -> String {
        String::from("FimshNotes")
    }
    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                notes: Notes::new("/Users/tymek/.fimshnotes/notes").unwrap(),
            },
            Command::none(),
        )
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::Event(e) => match e {
                Keyboard(e) => match e {
                    KeyPressed {
                        key_code: KeyCode::Escape,
                        modifiers: _,
                    } => {
                        // self.pallete = false;
                    }
                    CharacterReceived(':') => {
                        // self.pallete = true;
                    }

                    _ => {}
                },
                _ => {}
            },
            Self::Message::NewNote(s) => {}
            _ => {}
        }
        Command::none()
    }
    fn view(&self) -> Element<Self::Message> {
        Text::new("BALLS").into()
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        iced_native::subscription::events().map(Self::Message::Event)
    }
}
fn main() {
    App::run(Settings::default()).unwrap();
}

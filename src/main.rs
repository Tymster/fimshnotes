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
    pallete_mode: PalleteMode,
    pallete_input: String,
    error: Option<String>,
}
#[derive(Debug, Clone)]
enum AppMessage {
    Event(iced_native::Event),
    SetPalleteMode(PalleteMode),
    NewNote,
    RunCommand,
    UpdateCommand(String),
}
#[derive(Debug, Clone)]
enum PalleteMode {
    Hidden,
    Command,
    RenameNote,
    RenameFolder,
    NewNote,
    NewFolder,
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
                pallete_mode: PalleteMode::Hidden,
                pallete_input: String::new(),
                error: None,
            },
            Command::none(),
        )
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::NewNote => {
                match self.notes.new_note(&self.pallete_input) {
                    Ok(_) => {
                        self.error = Some(String::from("Made note"));
                        self.pallete_input = String::new();
                        self.pallete_mode = PalleteMode::Hidden;
                    }
                    Err(e) => self.error = Some(e.to_string()),
                };
                Command::none()
            }
            Self::Message::Event(e) => match e {
                Keyboard(e) => match e {
                    KeyPressed {
                        key_code: KeyCode::Escape,
                        modifiers: _,
                    } => Command::perform(async {}, |()| {
                        Self::Message::SetPalleteMode(PalleteMode::Hidden)
                    }),
                    KeyPressed {
                        key_code: KeyCode::N,
                        modifiers: Modifiers::CTRL,
                    } => Command::perform(async {}, |()| {
                        Self::Message::SetPalleteMode(PalleteMode::NewNote)
                    }),
                    KeyPressed {
                        key_code: KeyCode::F,
                        modifiers: Modifiers::CTRL,
                    } => Command::perform(async {}, |()| {
                        Self::Message::SetPalleteMode(PalleteMode::NewNote)
                    }),
                    CharacterReceived(':') => Command::perform(async {}, |()| {
                        Self::Message::SetPalleteMode(PalleteMode::Command)
                    }),
                    _ => Command::none(),
                },
                _ => Command::none(),
            },
            Self::Message::SetPalleteMode(mode) => {
                self.pallete_mode = mode;
                iced::widget::text_input::focus(iced::widget::text_input::Id::new("input"))
            }
            Self::Message::UpdateCommand(command) => {
                self.pallete_input = command;
                Command::none()
            }
            Self::Message::RunCommand => {
                println!("RIUnning comman dof  {}", self.pallete_input);
                self.pallete_input = String::new();
                Command::none()
            }
            _ => Command::none(),
        }
    }
    fn view(&self) -> Element<Self::Message> {
        let pallete = column(match self.pallete_mode {
            PalleteMode::Command => vec![TextInput::new("Command", &self.pallete_input)
                .on_submit(Self::Message::RunCommand)
                .on_input(Self::Message::UpdateCommand)
                .id(iced::widget::text_input::Id::new("input"))
                .into()],
            PalleteMode::NewNote => vec![TextInput::new("Name", &self.pallete_input)
                .on_input(Self::Message::UpdateCommand)
                .on_submit(Self::Message::NewNote)
                .id(iced::widget::text_input::Id::new("input"))
                .into()],
            _ => vec![],
        })
        .into();
        let error = match &self.error {
            Some(balls) => Text::new(balls),
            None => Text::new("COCK"),
        }
        .into();
        column(vec![pallete, error]).into()
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        iced_native::subscription::events().map(Self::Message::Event)
    }
}
fn main() {
    App::run(Settings::default()).unwrap();
}

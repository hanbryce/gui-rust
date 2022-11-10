use iced::{executor, Application, Command, Element, Settings, Subscription, Alignment};
use time::{OffsetDateTime};
use iced::widget::{Text, Button, button, Column};

pub fn main() -> iced::Result {
    Board::run(Settings::default())
}

struct Board {
    now: OffsetDateTime,
    button: button::State
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
}

impl Application for Board {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self { now: OffsetDateTime::now_utc(),
                button: button::State::new()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Black J")
    }

    fn update(&mut self, message: Message) -> Command<Message> {

        match message {
            Message::Tick => {
                self.now = OffsetDateTime::now_utc()
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
        .push(Text::new(self.now.to_string()))
        .push(Button::new(&mut self.button,Text::new("Up"))
            .on_press(Message::Tick)
        )
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    // fn subscription(&self) -> Subscription<Message> {
    //     iced::time::every(std::time::Duration::from_millis(500)).map(|_| {
    //         Message::Tick(
    //             time::OffsetDateTime::now_utc(),
    //         )
    //     })
    // }

}
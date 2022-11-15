use iced::{executor, Application, Command, Element, Settings, Subscription, Alignment, Theme, time as iced_time};
use time::{OffsetDateTime};
use iced::widget::{Text, Button, Column};

pub fn main() -> iced::Result {
    Board::run(Settings::default())
}

struct Board {
    now_click: OffsetDateTime,
    now_tick: OffsetDateTime
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Click,
    Tick,
}

impl Application for Board {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self { 
                now_click: OffsetDateTime::now_utc(),
                now_tick: OffsetDateTime::now_utc()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Black J")
    }

    fn update(&mut self, message: Message) -> Command<Message> {

        match message {
            Message::Click => {
                self.now_click = OffsetDateTime::now_utc()
            },
            Message::Tick => {
                self.now_tick = OffsetDateTime::now_utc()
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Column::new()
        .push(Text::new(self.now_click.to_string()))
        .push(Button::new(Text::new("Up"))
            .on_press(Message::Click)
        )
        .push(Text::new(self.now_tick.to_string()))
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_time::every(std::time::Duration::from_millis(500)).map(|_| {
            Message::Tick
        })
    }

}
use iced::widget;
#[derive(Default)]
struct Counter {
    count: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementCount,
    DecrementCount,
}

impl Counter {
    fn new() -> Self {
        Self {
            count: 0,
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::IncrementCount => {
                self.count += 1;
            }
            Message::DecrementCount => {
                self.count -= 1;
            }
        }

        iced::Task::none()
    }
    fn view(&self) -> iced::Element<'_, Message> {
        let children = widget::row![
            widget::button("+").on_press(Message::IncrementCount),
            widget::text!("Count: {}", self.count),
            widget::button("-").on_press(Message::DecrementCount)
        ];

        widget::container(children.spacing(10)).center(iced::Length::Fill).into()
    }
}

fn main() -> iced::Result {
    iced::application(Counter::new, Counter::update, Counter::view).title("Counter Example").run()
}

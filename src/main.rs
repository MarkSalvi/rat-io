use iced::widget::{button,column,text};
use iced::{Element};

#[derive(Default)]
struct Counter {
    value: i64,
}
#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

impl Counter{

    fn update(&mut self, message: Message){
        match message{
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self)-> Element<'_,Message>{
        // The buttons
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);

        // The number
        let counter = text(self.value);

        // The layout
        let interface = column![increment, counter, decrement];

        interface.into()
    }
}

fn main()-> iced::Result{
    iced::run("Ratio",Counter::update, Counter::view)
}

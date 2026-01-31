use iced::widget::{button,column,text,container,row,grid};
use iced::{Element,Fill,theme};
use iced_core::{window,Size};

#[derive(Default)]
struct Calculator {
    value: i64,
}
#[derive(Debug, Clone)]
enum Message {
    Canc,
    Calculate,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    DoubleZero,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Calculator{

    fn update(&mut self, message: Message){
        match message{
            _ => {}
        }
    }

    fn view(&self)-> Element<'_,Message>{

        //top display
        let display_value =  container(text(self.value).style(text::primary).width(Fill)).style(container::rounded_box);

        //First Row
        let canc = button("C");
        let calculate = button("=");

        //Second Row
        let seven = button("7");
        let eight = button("8");
        let nine = button("9");
        let division = button("÷");

        //Third Row
        let four = button("4");
        let five = button("5");
        let six = button("6");
        let multiplication = button("x");

        //Fourth Row
        let one = button("1");
        let two = button("2");
        let three = button("3");
        let minus = button("-");

        //Fifth Row
        let double_zero = button("00");
        let zero = button("0");
        let decimal_point = button(",");
        let plus = button("+");


        grid().width(4)
            .push(seven,eight,nine,division);

        // The layout
        container(column![display_value,
                                row![canc,calculate],
                                row![seven,eight,nine,division],
                                row![four,five,six,multiplication],
                                row![one,two,three,minus],
                                row![double_zero,zero,decimal_point,plus],
        ]).width(Fill).height(Fill).into()
    }
}



fn main()-> iced::Result {
    iced::application("Ratio", Calculator::update, Calculator::view).window(window::Settings {
            size: Size::new(300.0, 400.0),
            min_size: Some(Size::new(300.0, 400.0)),
            max_size: Some(Size::new(600.0, 800.0)),
            position: window::Position::Centered,
            resizable: true,
            decorations: true,
            transparent: true,
            ..Default::default()
        }).run()
}

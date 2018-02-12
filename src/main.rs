#![feature(duration_extras)]

pub mod elements;

#[macro_use] extern crate conrod;
extern crate time;
extern crate num;

use elements::{*, container::*, basic::*};



fn main() {
    let mut base_window = BaseWindow::new("Container".to_string(), 800, 600);

    let mut list = List::new(ListAlignment::Vertical);
    list.add_element(
        Pad::new(
            Button::new()
                .with_action_click( Box::new(|| {
                    println!("List -> Pad -> Button with const size");
                })),
            PadAlignment::TopLeft,
            PadElementSize::Absolute(200, 200)
        ).with_background(Background::Color(conrod::color::LIGHT_BLUE))
    );

    list.add_element(
        Pad::new(
            Button::new()
                .with_action_click(Box::new(||{
                    println!("List -> Button");
                })),
            PadAlignment::Center,
            PadElementSize::AbsoluteNeg(20,20)
        )
    );

    let mut sublist = List::new(ListAlignment::Horizontal);
    sublist.add_element(
        Button::new()
            .with_action_click(Box::new(|| {
                println!("List -> List -> Button 1")
            })),
    );
    sublist.add_element(
        Pad::new(
            Button::new()
                .with_action_click(Box::new(|| {
                    println!("List -> List -> Pad -> Button");
                }))
                .with_label("Stop.".to_string()),
            PadAlignment::Center,
            PadElementSize::Relative(0.5, 0.5)
        ).with_background(Background::Color(conrod::color::LIGHT_ORANGE))
    );
    sublist.add_element(
        Button::new()
            .with_action_click(Box::new(|| {
                println!("List -> List -> Button 2");
            }))
    );


    list.add_element(sublist);
    base_window.add_element(list);

    base_window.run(-1f64);
}
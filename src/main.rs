#![feature(duration_extras)]

pub mod elements;

#[macro_use] extern crate conrod;
extern crate time;
extern crate num;

use elements::{*, container::*, basic::*};



fn main() {
    let mut base_window = BaseWindow::new("Container".to_string(), 800, 800);

    let mut layers = Layers::new();

    let mut list = List::new(ListAlignment::Vertical);

    let mut sublist = List::new(ListAlignment::Horizontal);
    sublist.push(
        Button::new()
            .with_action_click(Box::new(|| {
                println!("List -> List -> Button 1")
            })),
    );
    sublist.push(
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
    sublist.push(
        Button::new()
            .with_action_click(Box::new(|| {
                println!("List -> List -> Button 2");
            }))
    );
    list.push(sublist);


    list.push(
        Pad::new(
            Button::new()
                .with_action_click(Box::new(||{
                    println!("List -> Button");
                })),
            PadAlignment::Center,
            PadElementSize::AbsoluteNeg(20,20)
        )
    );


    let mut inner_layer = Layers::new();
    inner_layer.push(Pad::new(
        Button::new()
            .with_action_click( Box::new(|| {
                println!("List -> Pad -> Button with const size");
            })),
        PadAlignment::TopLeft,
        PadElementSize::Absolute(200, 200) )
        .with_background(Background::Color(conrod::color::LIGHT_BLUE))
    );

    inner_layer.push(
        Label::new("Your Ads here!".to_string(), 60)
            .with_color(conrod::color::RED)
    );

    list.push(inner_layer);


    layers.push(list);

    layers.push(Pad::new(
        Button::new().with_action_click(Box::new(||{
            println!("Äktschöööööön!!!!");
        })).with_color(conrod::color::LIGHT_GREEN),
        PadAlignment::Center,
        PadElementSize::Relative(0.5, 0.4)
    ));


    base_window.add_element(layers);

    base_window.run(-1f64);
}
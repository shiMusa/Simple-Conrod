#![feature(duration_extras)]

pub mod elements;
pub mod clock;

#[macro_use] extern crate conrod;
extern crate time;
extern crate num;

use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{self, Sender, Receiver};



fn main() {
    use clock::*;

    let (clock_send, clock_recv): (Sender<ClockMsg>, Receiver<ClockMsg>) =
        mpsc::channel();
    let time = Arc::new(RwLock::new(0.0));
    let clock_core = clock::ClockCore::new(time.clone(),clock_recv);

    let cl = thread::spawn(move || {
        clock_core.run();
    });


    use clock;

    let w2 = thread::spawn(move || {

        let csend = clock_send.clone();

        use elements::*;
        use clock::*;
        let mut base_window = BaseWindow::new("Container".to_string(), 800, 600);

        //let clock = Clock::new(base_window.get_ui(), time.clone(), clock_send.clone());
        //base_window.add_element(Box::new(clock));


        let mut list = container::List::new(container::ListAlignment::Vertical);
        list.set_frame(Frame::new(300, 700));
        list.add_element(
            container::Pad::new(
                basic::Button::new()
                    .with_action_click( Box::new(|| {
                        println!("List -> Pad -> Button with const size");
                    })),
                container::PadAlignment::TopLeft,
                container::PadElementSize::Absolute(200, 200)
            ).with_background(Background::Color(conrod::color::LIGHT_BLUE))
        );
        list.add_element(
            basic::Button::new()
                .with_action_click(Box::new(||{
                    println!("List -> Button");
                }))
        );

        let mut sublist = container::List::new(container::ListAlignment::Horizontal);
        sublist.set_frame(Frame::new(300,300));
        sublist.add_element(
            basic::Button::new()
                .with_action_click(Box::new(|| {
                    println!("List -> List -> Button 1")
                })),
        );
        sublist.add_element(
            container::Pad::new(
                basic::Button::new()
                    .with_action_click(Box::new(move || {
                        println!("List -> List -> Pad -> Button");
                        let _ = csend.send(ClockMsg::Stop);
                    }))
                    .with_label("Stop.".to_string()),
                container::PadAlignment::Center,
                container::PadElementSize::Relative(0.5, 0.5)
            ).with_background(Background::Color(conrod::color::LIGHT_ORANGE))
        );
        sublist.add_element(
            basic::Button::new()
                .with_action_click(Box::new(|| {
                    println!("List -> List -> Button 2");
                }))
        );


        list.add_element(sublist);
        base_window.add_element(list);

        base_window.run(0f64);
    });
    let _ = w2.join();
    let _ = cl.join();
}






// keeping it just for reference...

/*
fn build_window(ui: &mut conrod::UiCell, ids: &Ids, link: &Link, sender: &Sender<Msg>) {
    let t = *link.data.read().unwrap();
    let mins = (t/60.0) as u8;
    let secs = t as u8 - 60*mins;


    widget::Text::new("This is now!")
        .middle_of(ui.window)
        .color(conrod::color::LIGHT_ORANGE)
        .font_size(64)
        .set(ids.text, ui);


    let mut rx = (t / (60.0 * 60.0) * 2.0 * std::f64::consts::PI).sin();
    let mut ry = (t / (60.0 * 60.0) * 2.0 * std::f64::consts::PI).cos();
    let style = widget::primitive::shape::Style::outline_styled(
        widget::primitive::line::Style::solid().thickness(5.0)
    );
    widget::Circle::styled(66.0, style)
        .x_y(rx*500.0,ry*500.0)
        .color(conrod::color::LIGHT_GREEN)
        .set(ids.circle_min, ui);

    widget::Line::new([0.0, 0.0], [rx*(500.0-66.0),ry*(500.0-66.0)])
        .thickness(5.0)
        .color(conrod::color::LIGHT_GREEN)
        .set(ids.line_min, ui);

    widget::Text::new(&format!("{:2}",mins))
        .x_y(rx*500.0,ry*500.0)
        .color(conrod::color::LIGHT_GREEN)
        .font_size(48)
        .set(ids.text_min, ui);


    rx = (t / 60.0 * 2.0 * std::f64::consts::PI).sin();
    ry = (t / 60.0 * 2.0 * std::f64::consts::PI).cos();
    widget::Circle::styled(100.0, style)
        .x_y(rx*200.0,ry*200.0)
        .color(conrod::color::LIGHT_BLUE)
        .set(ids.circle_sec, ui);

    widget::Line::new([0.0, 0.0], [rx*100.0,ry*100.0])
        .thickness(5.0)
        .color(conrod::color::LIGHT_BLUE)
        .set(ids.line_sec, ui);

    widget::Text::new(&format!("{:2}",secs))
        .x_y(rx*200.0,ry*200.0)
        .color(conrod::color::LIGHT_BLUE)
        .font_size(48)
        .set(ids.text_sec, ui);




    if widget::Button::new()
        .color(conrod::color::DARK_CHARCOAL)
        .mid_bottom_of(ui.window)
        .w_h(200.0,50.0)
        .label("toggle calculation")
        .set(ids.button, ui)
        .was_clicked() {

        let _ = sender.send(Msg::Toggle);
    }

    if widget::Button::new()
        .color(conrod::color::LIGHT_PURPLE)
        .bottom_right_of(ui.window)
        .w_h(200.0,50.0)
        .label("Rhai!")
        .set(ids.button_rhai, ui)
        .was_clicked() {

        println!("not implemented yet!");
        //rhai();
    }

    if widget::Button::new()
        .color(conrod::color::LIGHT_YELLOW)
        .bottom_left_of(ui.window)
        .w_h(200.0,50.0)
        .label("Dyon!")
        .set(ids.button_dyon, ui)
        .was_clicked() {

        println!("not implemented yet!");
        //dyon();
    }
}
*/
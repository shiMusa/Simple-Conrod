#![feature(duration_extras)]

pub mod elements;
pub mod clock;

#[macro_use] extern crate conrod;
extern crate time;

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
        use elements::*;
        use clock::*;
        let mut base_window = BaseWindow::new("Clock".to_string(), 800, 600);

        let clock = Clock::new(base_window.get_ui(), time, clock_send);
        base_window.add_element(Box::new(clock));

        base_window.run(120f64);
    });
    let _ = w2.join();
    let _ = cl.join();
}
























/*

extern crate rhai;
use rhai::{RegisterFn};



pub fn rhai() {
    let mut rhai_engine = rhai::Engine::new();

    use std::fmt::Display;
    fn showit<T: Display>(x: &mut T) -> () {
        println!("{}", x)
    }

    rhai_engine.register_fn("print", showit as fn(x: &mut i32) -> ());
    rhai_engine.register_fn("print", showit as fn(x: &mut i64) -> ());
    rhai_engine.register_fn("print", showit as fn(x: &mut u32) -> ());
    rhai_engine.register_fn("print", showit as fn(x: &mut u64) -> ());
    rhai_engine.register_fn("print", showit as fn(x: &mut f32) -> ());
    rhai_engine.register_fn("print", showit as fn(x: &mut f64) -> ());
    rhai_engine.register_fn("print", showit as fn(x: &mut bool) -> ());
    rhai_engine.register_fn("print", showit as fn(x: &mut String) -> ());

    match rhai_engine.eval_file::<()>("src\\main.rhai") {
        Ok(result) => println!("rhai : {:?}", result),
        Err(e) => println!("rhai error! {:?}", e),
    }

    /*
    match rhai_engine.call_fn::<String, Option<()>, _>("test".to_owned(), None) {
        Ok(_) => println!("funciton executed"),
        Err(e) => println!("rhai error! {:?}", e),
    }
    */
}


*/





/*


#[macro_use]
extern crate dyon;
use dyon::{error, Runtime, Module};

pub fn dyon(sender: &Sender<Msg>) {
    let mut dyon_runtime = Runtime::new();
    let mut module = Module::new();

    module.add(Arc::new("add_time".into()), add_time, Dfn{
        lts: vec![dyon::Lt::Default],
        tys: vec![dyon::Type::F64],
        ret: dyon::Type::Void
    });

    fn add_t(time : f64) {
        let _ = sender.send(Msg::Add(time));
    }
    // dyon functions
    dyon_fn!(fn add_time(time: f64) {
        add_t(time);
    });

    let _ = error(load("src\\main.dyon", &mut module));

    let _ = error(dyon_runtime.run(&Arc::new(dyon_module)));
}


*/



















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
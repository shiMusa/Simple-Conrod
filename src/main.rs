#![feature(duration_extras)]

pub mod elements;
pub mod clock;

#[macro_use] extern crate conrod;
extern crate time;

use conrod::{widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use conrod::backend::glium::glium::{self, Surface};

use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{self, Sender, Receiver};




#[derive(Clone)]
pub struct Link {
    data: Arc<RwLock<f64>>,
}

#[derive(Clone, Copy, PartialOrd, PartialEq)]
pub enum Msg {
    Close,
    Toggle,
    Add(f64),
}


pub struct System {
    link: Link,
    receiver: Receiver<Msg>,
}

impl System {
    pub fn new(link: Link, receiver: Receiver<Msg>) -> Self {
        System { link, receiver }
    }

    pub fn run(&self) {
        const FPS: f64 = 120.0;
        const DT_NS: u64 = (1.0e9/FPS) as u64;

        let mut running = true;
        let t0 = time::precise_time_ns();
        let mut old = t0;

        loop {
            match self.receiver.try_recv() {
                Ok(msg) => match msg {
                    Msg::Close => break,
                    Msg::Toggle => running = !running,
                    _ => (),
                },
                Err(_e) => ()
            }

            if running {
                let t = time::precise_time_ns() - t0;
                let mut linkdat = self.link.data.write().unwrap();
                *linkdat = (t as f64) * 1e-9;
            }


            let dt = time::precise_time_ns() - old;
            //let fdt = (dt as f64) * 1e-9;
            //println!("fps = {:8.0}, dt = {:.6}", 1.0/fdt, fdt);

            old = time::precise_time_ns();
            if DT_NS > dt {
                let sl = DT_NS - dt;
                thread::sleep(std::time::Duration::from_nanos(sl));
            }
        }
    }
}


fn main() {
    let link = Link {
        data: Arc::new(RwLock::new(0.0))
    };
    let (tx, rx): (Sender<Msg>, Receiver<Msg>) = mpsc::channel();



    let sys = System::new(link.clone(), rx);

    let handle = thread::spawn(move || {
        sys.run();
    });

    let l0 = link.clone();
    let tx0 = tx.clone();
    let w0 = thread::spawn(move || {
        create_winow(l0, tx0);
    });

    let l1 = link.clone();
    let tx1 = tx.clone();
    let w1 = thread::spawn(move || {
        create_winow(l1, tx1);
    });


    // clock --------------------------------------------------
    use clock::*;

    let (clock_send, clock_recv): (Sender<ClockMsg>, Receiver<ClockMsg>) =
        mpsc::channel();
    let time = Arc::new(RwLock::new(0.0));
    let clock_core = clock::ClockCore::new(time.clone(),clock_recv);

    let cl = thread::spawn(move || {
        clock_core.run();
    });


    use clock;
    use elements::*;

    let w2 = thread::spawn(move || {
        let mut cl = clock::Clock::new("Clock".to_string(), 800, 600);
        cl.setup(time, clock_send);
        cl.run(120.0);
    });


    let _ = w0.join();
    let _ = w1.join();
    let _ = w2.join();
    let _ = cl.join();
    let _ = handle.join().unwrap();
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






















widget_ids! (
    struct Ids{
        text,
        button,
        circle_sec,
        line_sec,
        text_sec,
        circle_min,
        line_min,
        text_min,
        button_rhai,
        button_dyon,
    }
);



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



pub fn create_winow(link: Link, sender: Sender<Msg>) {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    const FPS: f64 = 60.0;
    const DT_NS: u64 = (1.0e9/FPS) as u64;


    // build window
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello World from Conrod!")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(8);
    let display = glium::Display::new(
        window, context, &events_loop
    ).unwrap();





    // create conrod ui
    let mut ui =  conrod::UiBuilder::new(
        [WIDTH as f64, HEIGHT as f64]
    ).build();



    // Generate widget identifiers
    let ids = Ids::new(ui.widget_id_generator());



    // Add a font to the ui's font::map from file
    const FONT_PATH: &'static str =
        concat!(env!("CARGO_MANIFEST_DIR"),
                "\\assets\\fonts\\NotoSans\\NotoSans-Regular.ttf");
    println!("{}", FONT_PATH);
    ui.fonts.insert_from_file(FONT_PATH).unwrap();

    // connect conrod::render::Primitives to glium Surface
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // image mapping, here: none
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // events
    let mut events = Vec::new();
    let mut t0 = time::precise_time_ns();
    'render: loop {
        events.clear();

        //println!("looping around");

        // get new events after last frame
        events_loop.poll_events(|event| {
            events.push(event);
        });

        //println!("events there???");
        // if no events, wait
        /*
        if events.is_empty() {
            //println!("not yet T.T");
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        } else {
            //println!("YES!!!");
        }*/


        let mut update = false;

        // process events
        for event in events.drain(..) {

            use conrod::glium::glutin::{Event, WindowEvent, KeyboardInput, VirtualKeyCode};

            match event.clone() {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Closed |
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput{
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => break 'render,
                        _ => (),
                    }
                },
                _ => (),
            };

            // convert winit event to conrod input
            let input = match conrod::backend::winit::convert_event(event, &display) {
                None => continue,
                Some(input) => input,
            };

            // handle input
            ui.handle_event(input);

            update = true;
        }

        let time_diff = time::precise_time_ns() - t0;
        if time_diff >= DT_NS {
            //println!("{:?}", time_diff);
            ui.needs_redraw();
            t0 = time::precise_time_ns();
            update = true;
        }

        if update {
            // set widgets
            let ui = &mut ui.set_widgets();
            // add some stuff
            /*
            widget::Text::new(&format!("This is now! {}", *link.data.read().unwrap()))
                .middle_of(ui.window)
                .color(conrod::color::LIGHT_ORANGE)
                .font_size(64)
                .set(ids.text, ui);
            */
            build_window(ui, &ids, &link, &sender);
        }

        // draw ui if changed
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }

    let _ = sender.send(Msg::Close);
}












































////////////////////////////////////
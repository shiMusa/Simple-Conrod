

pub mod container;
pub mod basic;


use conrod;
use conrod::backend::glium::glium::{self, Surface};
use time;
use num::{Num, NumCast};
use std::ops::{Add, Sub, Mul, Div};


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    pub x: T, pub y: T
}

impl<T> Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    pub fn zero() -> Vec2<T> {
        Vec2{
            x: T::from(0).unwrap(), y: T::from(0).unwrap(),
        }
    }

    pub fn el_mul(self, other: Vec2<T>) -> Vec2<T> {
        Vec2{
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl<T> Add for Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    type Output = Vec2<T>;

    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    type Output = Vec2<T>;

    fn sub(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Div<T> for Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Vec2<T> {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> Mul<T> for Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Vec2<T> {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}











#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Frame<T> where  T: Num + NumCast + PartialOrd + Copy {
    pub p0: Vec2<T>, pub p1: Vec2<T>,
}
impl<T> Frame<T> where T: Num + NumCast + PartialOrd + Copy {
    pub fn new(width: T, height: T) -> Self {
        Frame{
            p0: Vec2{x: T::from(0).unwrap(), y: T::from(0).unwrap()},
            p1: Vec2{x: width, y: height}
        }
    }

    pub fn width(&self) -> T {
        self.p1.x - self.p0.x
    }
    pub fn height(&self) -> T {
        self.p1.y - self.p0.y
    }
    pub fn max_dim(&self) -> T {
        if self.width() >= self.height() { return self.width() } else { return self.height() }
    }
    pub fn min_dim(&self) -> T {
        if self.width() <= self.height() { return self.width() } else { return self.height() }
    }
    pub fn center(&self) -> Vec2<T> {
        (self.p1 + self.p0) / (T::from(2).unwrap())
    }
    pub fn size(&self) -> Vec2<T> {
        self.p1 - self.p0
    }
    //pub fn center_left(&self) -> Vec2<T>
}













use std::i32;

pub trait Element {
    fn setup(&mut self, ui: &mut conrod::Ui);

    fn stop(&self) {}
    fn build_window(&self, ui: &mut conrod::UiCell);

    fn get_frame(&self) -> Frame<i32>;
    fn set_frame(&mut self, frame: Frame<i32>);

    fn get_min_size(&self) -> Vec2<i32> {
        Vec2::zero()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        Vec2{ x: i32::MAX, y: i32::MAX }
    }

    fn set_window_center(&mut self, center: Vec2<i32>);
}



pub trait Clickable {
    fn with_action_click(self, fun: Box<Fn()>) -> Box<Self>;
}

pub trait Labelable {
    fn with_label(self, label: String) -> Box<Self>;
}



pub enum Background {
    None,
    Color(conrod::Color),
}

pub trait Backgroundable {
    fn with_background(self, bg: Background) -> Box<Self>;
}
















pub struct BaseWindow {
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    renderer: conrod::backend::glium::Renderer,
    image_map: conrod::image::Map<glium::texture::Texture2d>,
    ui: conrod::Ui,

    window_center: Vec2<i32>,

    element: Option<Box<Element>>,
}

impl BaseWindow {

    fn setup(&mut self) {
        if let Some(ref mut el) = self.element {
            el.setup(&mut self.ui);
        }
    }

    /*
    pub fn get_ui(&mut self) -> &mut conrod::Ui {
        &mut self.ui
    }
    */


    pub fn add_element(&mut self, mut element: Box<Element>) {
        element.set_window_center(self.window_center);
        self.element = Some(element);
    }


    pub fn new(title: String, width: u32, height: u32) -> Self {
        // build window
        let events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(8);
        let display = glium::Display::new(
            window, context, &events_loop
        ).unwrap();



        // create conrod ui
        let mut ui =  conrod::UiBuilder::new(
            [width as f64, height as f64]
        ).build();


        // Add a font to the ui's font::map from file
        const FONT_PATH: &'static str =
            concat!(env!("CARGO_MANIFEST_DIR"),
                "\\assets\\fonts\\NotoSans\\NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(FONT_PATH).unwrap();

        // connect conrod::render::Primitives to glium Surface
        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // image mapping, here: none
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();


        BaseWindow {
            events_loop,
            display,
            renderer,
            image_map,
            ui,
            element: None,
            window_center: Vec2{x: (width as f64/2f64) as i32, y: (height as f64/2f64) as i32}
        }
    }

    pub fn run(&mut self, fps: f64) {

        self.setup();

        let dt_ns = (1.0e9/fps) as u64;

        // events
        let mut events = Vec::new();
        let mut t0 = time::precise_time_ns();

        //println!("loop is starting ...............................");
        'render: loop {
            events.clear();

            // get new events after last frame
            self.events_loop.poll_events(|event| {
                events.push(event);
            });


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
                            WindowEvent::Resized(mut w, mut h) => {
                                //println!("resized: ({}, {})",w,h);
                                if let Some(ref mut el) = self.element {
                                    let min = el.get_min_size();
                                    let max = el.get_max_size();

                                    if w > max.x as u32 { w = max.x as u32; }
                                    if w < min.x as u32 { w = min.x as u32; }
                                    if h > max.y as u32 { h = max.y as u32; }
                                    if h < min.y as u32 { h = min.y as u32; }

                                    self.window_center = Vec2{x: (w as f64/2f64) as i32, y: (h as f64/2f64) as i32};
                                    el.set_frame(Frame::new(w as i32,h as i32));
                                    el.set_window_center(self.window_center);
                                } else {
                                    self.window_center = Vec2{x: (w as f64/2f64) as i32, y: (h as f64/2f64) as i32};
                                }
                            }
                            _ => (),
                        }
                    },
                    _ => (),
                };

                // convert winit event to conrod input
                let input = match conrod::backend::winit::convert_event(event, &self.display) {
                    None => continue,
                    Some(input) => input,
                };

                // handle input
                self.ui.handle_event(input);

                update = true;
            }

            let time_diff = time::precise_time_ns() - t0;
            if time_diff >= dt_ns {
                self.ui.needs_redraw();
                t0 = time::precise_time_ns();
                update = true;
            }

            if update {
                let ui = &mut self.ui.set_widgets();
                if let Some(ref mut el) = self.element {
                    el.build_window(ui);
                }
            }

            // draw ui if changed
            if let Some(primitives) = self.ui.draw_if_changed() {
                self.renderer.fill(&self.display, primitives, &self.image_map);
                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
                target.finish().unwrap();
            }
        }

        if let Some(ref el) = self.element {
            el.stop();
        }
    }
}
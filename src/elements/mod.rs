

pub mod container;
pub mod basic;


use conrod;
use conrod::backend::glium::glium::{self, Surface};
use time;
use num::{Num, NumCast};
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Debug, Formatter, Result};





static DEBUG: bool = false;

























#[derive(Debug, Copy, Clone)]
pub enum RingElementSize<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    Absolute(T),
    Relative(f64),
}
impl<T> RingElementSize<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    pub fn to_value(& self, cmp_to: T) -> T {
        match self {
            &RingElementSize::Absolute(x) => x,
            &RingElementSize::Relative(x) => T::from(cmp_to.to_f64().unwrap() * x).unwrap(),
        }
    }
}










#[derive(Debug, Copy, Clone)]
pub struct RingElement<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    min: RingElementSize<T>,
    max: RingElementSize<T>,
    size: RingElementSize<T>
}
impl<T> RingElement<T> where T: Num + NumCast + PartialOrd + Copy + Debug{
    pub fn new_with_min_max(
        min: RingElementSize<T>,
        max: RingElementSize<T>) -> Self {
        RingElement {
            min, max, size: RingElementSize::Relative(0.5)
        }
    }

    pub fn new() -> Self {
        RingElement::new_with_min_max(
            RingElementSize::Relative(0.0),
            RingElementSize::Relative(1.0),
        )
    }

    pub fn is_at_max(&self, cmp: T) -> bool {
        let size = self.size.to_value(cmp);
        let max = self.max.to_value(cmp);

        if DEBUG {
            println!("is at max? size {:?}, max {:?}", size, max);
        }

        if size >= max {
            if DEBUG { println!("   yes.");}
            true
        } else {
            if DEBUG { println!("   no.");}
            false
        }
    }

    pub fn shrink_to_min(&mut self) {
        if DEBUG { println!("shrinking size {:?} to min {:?}", self.size, self.min);}
        self.size = self.min;
        if DEBUG { println!("shrinked? size {:?} to min {:?}", self.size, self.min);}
    }

    pub fn grow(&mut self, cmp: T, grow_by: T) -> T {
        if DEBUG { println!("growing size {:?}, max {:?} by {:?}", self.size.to_value(cmp), self.max.to_value(cmp), grow_by);}
        if self.is_at_max(cmp) {
            if DEBUG { println!("cannot grow. already at max.");}
            return grow_by
        }
        let s = self.size.to_value(cmp);
        self.size = RingElementSize::Absolute(s + grow_by);

        if self.is_at_max(cmp) {
            let rem = self.size.to_value(cmp) - self.max.to_value(cmp);
            self.size = self.max;
            if DEBUG { println!("grown? size {:?}, max {:?}, rem {:?}", self.size.to_value(cmp), self.max.to_value(cmp), rem);}
            return rem;
        }
        if DEBUG { println!("everything absorbed.");}
        T::from(0).unwrap()
    }
}














#[derive(Clone)]
pub struct Ring<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    size: T,
    elements: Vec<RingElement<T>>,
    buffer: RingElement<T>,
}

impl<T> Ring<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    pub fn new_with_size(size: T) -> Self {
        // buffer in case all min-sizes are reached.
        // the ring is always filled out!
        Ring {
            size,
            elements: Vec::new(),
            buffer: RingElement::new()
        }
    }
    pub fn new() -> Self {
        Ring::new_with_size(T::from(1).unwrap())
    }

    pub fn push(&mut self, element: RingElement<T>) {
        let n = self.elements.len();
        self.insert(n, element);
    }

    pub fn insert(&mut self, index: usize, element: RingElement<T>) {
        // check if it fits at all
        let mut min = T::from(0).unwrap();

        for el in &self.elements {
            min = min + el.min.to_value(self.size);
        }
        if DEBUG { println!("minimal size for all elements {:?}", min);}
        let mut rem = self.size - min;

        let elmin = element.min.to_value(self.size);

        if DEBUG {
            if rem < elmin {
                println!("ERROR: Cannot insert element into Ring: no size left. Ignoring insert...");
                println!("       size left: {:?}, min size of element: {:?}", rem, elmin);
            }

            if rem < element.size.to_value(self.size) {
                println!("shrinking element to fit in leftover space...")
            }
        }

        // add element since possible
        rem = rem - element.min.to_value(self.size);

        if index >= self.elements.len() {
            self.elements.push(element);
        } else {
            self.elements.insert(index, element);
        }



        let num = self.elements.len();

        // shrink all elements before expanding again
        self.buffer.shrink_to_min();
        for el in &mut self.elements {
            el.shrink_to_min();
        }

        // gradually expand as long as there is space left
        if DEBUG { println!("starting remnant {:?}", rem);}

        while rem > T::from(0).unwrap() {
            // calculate remnant space in this iteration and the amount to grow
            // TODO implement weights for each element
            let rem_grow = T::from( rem.to_f64().unwrap() / num as f64).unwrap();
            rem = T::from(0).unwrap();

            // grow and store leftover
            for k in 0..num {
                rem = rem + self.elements[k].grow(self.size, rem_grow);
            }
            if DEBUG { println!("    remnant {:?}", rem);}

            // all max_sizes reached?
            let mut all_max = true;
            for el in &self.elements {
                if !el.is_at_max(self.size) {
                    all_max = false;
                }
            }
            if all_max {
                if DEBUG { println!("every element at max. Filling buffer...");}
                self.buffer.size = RingElementSize::Absolute(rem);
                break
            }
        }
    }
}

#[allow(unused_must_use)]
impl<T> Debug for Ring<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Ring: size {:?}", self.size);
        writeln!(f, "    buffer: size {:?}", self.buffer.size);
        for n in 0..self.elements.len() {
            let min = self.elements[n].min.to_value(self.size);
            let size = self.elements[n].size.to_value(self.size);
            let max = self.elements[n].max.to_value(self.size);

            writeln!(f, "    min: {:?}, size: {:?}, max: {:?}", min, size, max);
        }
        writeln!(f, "")
    }
}

































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

            if fps > 0.0 {
                let time_diff = time::precise_time_ns() - t0;
                if time_diff >= dt_ns {
                    self.ui.needs_redraw();
                    t0 = time::precise_time_ns();
                    update = true;
                }
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
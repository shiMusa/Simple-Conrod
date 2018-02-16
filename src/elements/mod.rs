

pub mod container;
pub mod basic;
pub mod action;


use conrod;
use conrod::backend::glium::glium::{self, Surface};
use time;
use num::{Num, NumCast};
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Debug, Formatter, Result};
use std::sync::mpsc::{Sender, Receiver};



const DEBUG: bool = false;




use elements::action::*;




















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

    pub fn get_min(&self, cmp: T) -> T {
        let min = self.min.to_value(cmp);
        let max = self.max.to_value(cmp);

        if max < min { return max; }
        min
    }

    pub fn get_max(&self, cmp: T) -> T {
        let min = self.min.to_value(cmp);
        let max = self.max.to_value(cmp);

        if max < min { return min; }
        max
    }

    pub fn get_size(&self, cmp: T) -> T {
        self.size.to_value(cmp)
    }

    pub fn is_at_max(&self, cmp: T) -> bool {
        let size = self.size.to_value(cmp);
        let max = self.get_max(cmp);

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

    pub fn shrink_to_min(&mut self, cmp: T) {
        if DEBUG {
            println!("shrinking size {:?} to min {:?}", self.size, self.min);
        }

        self.size = if self.get_min(cmp) > self.get_max(cmp) {
            if DEBUG { println!("min > max, so set to max");}
            self.max
        } else {
            self.min
        };
        if DEBUG { println!("shrinked? size {:?} to min {:?}", self.size, self.min);}
    }

    pub fn grow(&mut self, cmp: T, grow_by: T) -> T {
        if DEBUG { println!("growing size {:?}, max {:?} by {:?}", self.size.to_value(cmp), self.get_max(cmp), grow_by);}
        if self.is_at_max(cmp) {
            if DEBUG { println!("cannot grow. already at max.");}
            return grow_by
        }
        let s = self.size.to_value(cmp);
        self.size = RingElementSize::Absolute(s + grow_by);

        if self.is_at_max(cmp) {
            let rem = self.size.to_value(cmp) - self.get_max(cmp);
            self.size = self.max;
            if DEBUG { println!("grown? size {:?}, max {:?}, rem {:?}", self.size.to_value(cmp), self.get_max(cmp), rem);}
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

    pub fn get(&self, index: usize) -> T {
        if index == self.elements.len() {
            T::zero()
        } else {
            self.elements[index].get_size(self.size)
        }
    }

    pub fn get_sum(&self, mut index: usize) -> T {
        let mut res = T::zero();
        if index == 0 { return res; }
        if index > self.elements.len() {
            index = self.elements.len();
        }

        for i in 0..index {
            res = res + self.get(i);
        }
        res
    }

    pub fn push(&mut self, element: RingElement<T>) {
        let n = self.elements.len();
        self.insert(n, element);
    }

    pub fn insert(&mut self, index: usize, element: RingElement<T>) {
        // check if it fits at all
        let mut min = T::zero();

        for el in &self.elements {
            min = min + el.get_min(self.size);
        }
        if DEBUG { println!("minimal size for all elements {:?}", min);}
        let rem = self.size - min;

        if DEBUG {
            let elmin = element.get_min(self.size);

            if rem < elmin {
                println!("ERROR: Cannot insert element into Ring: no size left. Ignoring insert...");
                println!("       size left: {:?}, min size of element: {:?}", rem, elmin);
            }

            if rem < element.size.to_value(self.size) {
                println!("shrinking element to fit in leftover space...")
            }
        }

        // add element since possible
        //rem = rem - element.min.to_value(self.size);

        if index >= self.elements.len() {
            self.elements.push(element);
        } else {
            self.elements.insert(index, element);
        }

        self.rescale_elements();
    }

    pub fn pop(&mut self) -> Option<RingElement<T>> {
        let el = self.elements.pop();
        self.rescale_elements();
        el
    }

    pub fn remove(&mut self, index: usize) -> RingElement<T> {
        let el = self.elements.remove(index);
        self.rescale_elements();
        el
    }

    pub fn resize(&mut self, size: T) {
        self.size = size;
        self.rescale_elements();
    }

    fn rescale_elements(&mut self) {
        if DEBUG { println!("Ring: rescale_elements...");}
        let num = self.elements.len();

        // shrink all elements before expanding again
        self.buffer.shrink_to_min(self.size);
        let mut min = T::from(0).unwrap();
        for el in &mut self.elements {
            el.shrink_to_min(self.size);
            if DEBUG { println!("{:?}", el); }
            min = min + el.get_min(self.size);
        }
        let mut rem = self.size - min;

        // gradually expand as long as there is space left
        if DEBUG { println!("starting remnant {:?}", rem);}
        let mut num_growable = num;

        let mut checked = Vec::with_capacity(num);
        for _ in 0..num { checked.push(false); }

        while rem > T::from(0).unwrap() {
            if DEBUG { println!(" --- loop --- {}", num_growable);}

            // calculate remnant space in this iteration and the amount to grow
            // TODO implement weights for each element
            let rem_grow = T::from( rem.to_f64().unwrap() / (num_growable as f64)).unwrap();

            // consider rem_grow = 0.5 for int, so actually rem_grow = 0, but still space left
            let mut sub_one = rem - rem_grow * T::from(num_growable).unwrap();

            // grow
            let mut el_size = T::zero();
            for k in 0..num {
                let el = &mut self.elements[k];
                if !checked[k] {
                    if sub_one > T::zero() {
                        el.grow(self.size, rem_grow + T::one());
                        sub_one = sub_one - T::one();
                    } else {
                        el.grow(self.size, rem_grow);
                    }
                }
                el_size = el_size + el.get_size(self.size);
            }
            rem = self.size - el_size;
            if DEBUG { println!("    remnant {:?}", rem);}

            // all max_sizes reached?
            let mut all_max = true;
            num_growable = num;
            for k in 0..num {
                let el = &mut self.elements[k];
                if !checked[k] && !el.is_at_max(self.size) {
                    all_max = false;
                } else {
                    checked[k] = true;
                    num_growable -= 1;
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
























pub enum Dim {
    Absolute(i32),
    Relative(f64),
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
    pub fn new_with_size(width: T, height: T) -> Self {
        Frame{
            p0: Vec2{x: T::zero(), y: T::zero()},
            p1: Vec2{x: width, y: height}
        }
    }

    pub fn new() -> Self {
        let one = T::one();
        Frame::new_with_size(one, one)
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
    fn is_setup(&self) -> bool;

    fn stop(&self) {}
    fn build_window(&self, ui: &mut conrod::UiCell);

    fn get_frame(&self) -> Frame<i32>;
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>);

    fn get_min_size(&self) -> Vec2<i32> {
        Vec2::zero()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        Vec2{ x: i32::MAX, y: i32::MAX }
    }

    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool);
}


pub trait Clickable {
    fn with_action_click(self, fun: Box<Fn()>) -> Box<Self>;
}

pub trait Labelable {
    fn with_label(self, label: String) -> Box<Self>;
    fn set_label(&mut self, label: String);
}

pub trait Colorable {
    fn with_color(self, color: conrod::Color) -> Box<Self>;
    fn set_color(&mut self, color: conrod::Color);
}


pub enum Background {
    None,
    Color(conrod::Color),
}

pub trait Backgroundable {
    fn with_background(self, bg: Background) -> Box<Self>;
    fn set_background(&mut self, bg: Background);
}



























pub struct BaseWindow {
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    renderer: conrod::backend::glium::Renderer,
    image_map: conrod::image::Map<glium::texture::Texture2d>,
    ui: conrod::Ui,

    element: Option<Box<Element>>,
    receiver: Option<Receiver<ActionMsg>>,
    senders: Vec<Sender<ActionMsg>>,
}

impl BaseWindow {

    fn setup(&mut self) {
        if let Some(ref mut el) = self.element {
            el.setup(&mut self.ui);
        }
    }


    pub fn add_element(&mut self, element: Box<Element>) {
        self.element = Some(element);
    }

    pub fn add_receiver(&mut self, receiver: Receiver<ActionMsg>) {
        self.receiver = Some(receiver);
    }

    pub fn add_sender(&mut self, sender: Sender<ActionMsg>) {
        self.senders.push(sender);
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
            receiver: None,
            senders: Vec::new(),
        }
    }

    pub fn run(&mut self, fps: f64) {

        self.setup();

        let dt_ns = (1.0e9/fps) as u64;

        // events
        let mut events = Vec::new();
        let mut t0 = time::precise_time_ns();

        let mut window_frame = Frame::new();

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

                                // TODO is this limit necessary?
                                if let Some(ref mut el) = self.element {
                                    let min = el.get_min_size();
                                    let max = el.get_max_size();

                                    if w > max.x as u32 { w = max.x as u32; }
                                    if w < min.x as u32 { w = min.x as u32; }
                                    if h > max.y as u32 { h = max.y as u32; }
                                    if h < min.y as u32 { h = min.y as u32; }

                                    window_frame = Frame::new_with_size(w as i32,h as i32);
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

            // check for fps forced update
            if fps > 0.0 {
                let time_diff = time::precise_time_ns() - t0;
                if time_diff >= dt_ns {
                    self.ui.needs_redraw();
                    t0 = time::precise_time_ns();
                    update = true;
                }
            }

            // check if msgs have to be processed and transmit through chain
            if let &Some(ref receiver) = &self.receiver {
                'receive: loop {
                    match receiver.try_recv() {
                        Ok(msg) => {
                            if DEBUG { println!("message received: {:?}", msg); }
                            if let Some(ref mut el) = self.element {
                                el.transmit_msg(msg.clone(), false);
                            }
                            update = true;
                            for sender in &mut self.senders {
                                let _ = sender.send(msg.clone());
                            }
                        },
                        _ => break 'receive
                    }
                }
            }

            if update {
                if let Some(ref mut el) = self.element {
                    if !el.is_setup() {
                        el.setup(&mut self.ui);
                    }
                }

                let ui = &mut self.ui.set_widgets();
                if let Some(ref mut el) = self.element {
                    el.set_frame(window_frame, window_frame.center());
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
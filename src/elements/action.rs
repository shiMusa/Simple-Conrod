
use conrod;

use elements::*;



#[allow(dead_code)]
const DEBUG: bool = false;
















use std::sync::mpsc::{Sender};


#[derive(Debug, Clone)]
pub enum ActionMsgData {
    Mouse(f64,f64),
    Click,
    Text(String),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I64(i64),
    I32(i32),
    F64(f64),
    F32(f32),
    Usize(usize),
    Exit,

    Update,
    None,
}

#[derive(Debug, Clone)]
pub struct ActionMsg {
    pub sender_id: String,
    pub msg: ActionMsgData
}
impl ActionMsg {
    pub fn empty() -> Self {
        ActionMsg {
            sender_id: "".to_string(),
            msg: ActionMsgData::None
        }
    }
}


pub trait ActionSendable {
    fn with_id(self, id: String) -> Box<Self>;
    fn with_sender(self, sender: Sender<ActionMsg>) -> Box<Self>;
}
























use time::precise_time_ns;




pub trait Animateable : Element {
    fn is_size_animateable(&self) -> bool;
    fn is_position_animateable(&self) -> bool;

    fn animate_size(&mut self, x: Dim, y: Dim);
    fn animate_position(&mut self, x: Dim, y: Dim);

    fn reset(&mut self);
}


pub enum AnimationFunction {
    Size(Box<Fn(f64) -> (Dim,Dim)>),
    Position(Box<Fn(f64) -> (Dim,Dim)>),
    No
}











pub struct Animation {
    start_time: u64,
    functions: Vec<AnimationFunction>,

    running: bool,
    duration: f64,
}

impl Animation {
    pub fn new(duration: f64) -> Self {
        Animation {
            start_time: 0,
            functions: Vec::new(),

            running: false,
            duration,
        }
    }

    pub fn with_function(mut self, function: AnimationFunction) -> Animation {
        self.functions.push(function);
        self
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    fn time(&self) -> f64 {
        (precise_time_ns() - self.start_time) as f64 * 1e-6
    }
}

impl<'a, A> FnOnce<(&'a mut Box<A>,)> for Animation where A: Animateable {
    type Output = (); // time in ms
    extern "rust-call" fn call_once(mut self, args: (&'a mut Box<A>,)) {
        self.call_mut(args)
    }
}

impl<'a, A> FnMut<(&'a mut Box<A>,)> for Animation where A: Animateable {
    extern "rust-call" fn call_mut(&mut self, args: (&'a mut Box<A>,)) {

        // start animation if not already running
        if !self.running {
            self.start_time = precise_time_ns();
            self.running = true;
        }

        // stop animation when duration reached
        if self.time() > self.duration {
            self.running = false;
            (args.0).reset();
        }

        // if running: animate
        if self.running {
            let anim = args.0;

            let t = self.time();
            for fun in &self.functions {
                match fun {
                    &AnimationFunction::Size(ref size_fun) => {
                        let (x,y) = size_fun(t);
                        anim.animate_size(x,y);
                    }
                    _ => ()
                }
            }
        }
    }
}












pub struct MultiSocket<E: Animateable> {
    is_setup: bool,
    pub element: Box<E>,

    functions: Vec<(ActionMsg, Box<Animation>)>,
}

impl<E> MultiSocket<E> where E: Animateable {
    pub fn new(element: Box<E>) -> Box<Self> {
        Box::new(MultiSocket {
            is_setup: false,
            element,
            functions: Vec::new()
        })
    }

    pub fn push(&mut self, msg_type: ActionMsg, receive: Box<Animation>) {
        self.functions.push( (msg_type, receive) );
    }

    pub fn with_action_receive(mut self, msg_type: ActionMsg, receive: Box<Animation>) -> Box<Self> {
        self.push( msg_type, receive );
        Box::new(self)
    }
}


impl<E> Element for MultiSocket<E> where E: Animateable {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.element.setup(ui);
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool {
        let mut res = true;
        for &(ref msg, ref anim) in &self.functions {
            if anim.is_running() {
                //anim(&self.element);
                res = false;
            }
        }

        self.is_setup && self.element.is_setup()
    }

    fn stop(&self) {
        self.element.stop();
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {
        self.element.build_window(ui);
    }

    fn get_frame(&self) -> Frame<i32> {
        self.element.get_frame()
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.element.set_frame(frame, window_center);
    }

    fn get_min_size(&self) -> Vec2<i32> {
        self.element.get_min_size()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        self.element.get_max_size()
    }
    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) {
        // first MultiSocket, then content
        use std::mem::discriminant;

        match msg.msg {
            ActionMsgData::Update => {
                for &mut (ref mut _msg, ref mut anim) in &mut self.functions {
                    if anim.is_running() {
                        anim(&mut self.element);
                    }
                }
            },
            _ => ()
        }

        for &mut (ref mut m, ref mut fun) in &mut self.functions {
            if m.sender_id == msg.sender_id &&
                discriminant(&m.msg) == discriminant(&msg.msg) {
                //let mut anim: Animation = fun;
                println!("MultiSocket: execute animation --------");
                fun(&mut self.element);
            }
        }
        if !stop {
            self.element.transmit_msg(msg, false);
        }
    }
}





































pub struct Socket<E: Element> {
    is_setup: bool,
    element: Box<E>,
    receive: Box<Fn(&mut E, ActionMsg)>,
}
impl<E> Socket<E> where E: Element {

    pub fn new(element: Box<E>) -> Box<Self> {
        Box::new(Socket {
            is_setup: false,
            element,
            receive: Box::new(|_,_|{}),
        })
    }

    pub fn with_action_receive(mut self, fun: Box<Fn(&mut E, ActionMsg)>) -> Box<Self> {
        self.receive = fun;
        Box::new(self)
    }
}

impl<E> Element for Socket<E> where E: Element {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.element.setup(ui);
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool {
        self.is_setup && self.element.is_setup()
    }

    fn stop(&self) {
        self.element.stop();
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {
        self.element.build_window(ui);
    }

    fn get_frame(&self) -> Frame<i32> {
        self.element.get_frame()
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.element.set_frame(frame, window_center);
    }

    fn get_min_size(&self) -> Vec2<i32> {
        self.element.get_min_size()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        self.element.get_max_size()
    }
    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) {
        // first socket, then content
        (self.receive)(&mut self.element, msg.clone());
        if !stop {
            self.element.transmit_msg(msg, false);
        }
    }
}

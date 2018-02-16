

use conrod;

use elements::*;



#[allow(dead_code)]
const DEBUG: bool = false;
















use std::sync::mpsc::{Sender};


#[derive(Debug, Clone)]
pub enum ActionMsgData {
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
}

#[derive(Debug, Clone)]
pub struct ActionMsg {
    pub sender_id: String,
    pub msg: ActionMsgData
}


pub trait ActionSendable {
    fn with_id(self, id: String) -> Box<Self>;
    fn with_sender(self, sender: Sender<ActionMsg>) -> Box<Self>;
}








use time::precise_time_ns;



pub struct Action<E: Element> {
    start_time: u64
}

impl<E> Action<E> where E: Element {
    pub fn new() -> Self {
        Action { start_time: precise_time_ns()}
    }
}

impl<'a, E> FnOnce<&'a mut E, ActionMsg> for Action<E> {
    type Output = f64; // time in ms
    extern "rust-call" fn call_once(mut self, args: ()) -> f64 {
        self.call_mut(args)
    }
}

impl<'a, E> FnMut<&'a mut E, ActionMsg> for Action<E> {
    extern "rust-call" fn call_mut(&mut self, args: ()) -> f64 {
        self.call(args)
        //(precise_time_ns() - self.start_time) as f64 * 1e-6
    }
}

impl<E> Fn<&mut E, ActionMsg> for Action<E> {
    extern "rust-call" fn call(&self, _args: ()) -> f64 {
        (precise_time_ns() - self.start_time) as f64 * 1e-6
    }
}












type Func<E> = Box<Fn(&mut E, ActionMsg)>;


pub struct MultiSocket<E: Element> {
    is_setup: bool,
    pub element: Box<E>,

    functions: Vec<(ActionMsg, Func<E>)>,
}

impl<E> MultiSocket<E> where E: Element {
    pub fn new(element: Box<E>) -> Box<Self> {
        Box::new(MultiSocket {
            is_setup: false,
            element,
            functions: Vec::new()
        })
    }

    pub fn push(&mut self, msg_type: ActionMsg, receive: Func<E>) {
        self.functions.push( (msg_type, receive) );
    }

    pub fn with_action_receive(mut self, msg_type: ActionMsg, receive: Func<E>) -> Box<Self> {
        self.push( msg_type, receive );
        Box::new(self)
    }
}


impl<E> Element for MultiSocket<E> where E: Element {
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
        // first MultiSocket, then content
        use std::mem::discriminant;
        for &(ref m, ref fun) in &self.functions {
            if m.sender_id == msg.sender_id &&
                discriminant(&m.msg) == discriminant(&msg.msg) {

                (fun)(&mut self.element, msg.clone());
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



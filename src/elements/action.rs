

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




















pub struct Animation<E: Element> {
    is_setup: bool,
    pub element: Box<E>,
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





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










use std::cell::{RefCell};
use std::rc::Rc;

pub trait Linkable<E: Element> : Element {
    fn set_element_link(&mut self, element: Rc<RefCell<Box<E>>>);
}

pub struct Link<E: Element> {
    is_setup: bool,
    element: Rc<RefCell<Box<E>>>,
    links: Vec<Box<Linkable<E>>>,
}
impl<E> Link<E> where E: Element {
    pub fn new(element: Box<E>) -> Box<Self> {
        Box::new(Link{
            is_setup: false,
            element: Rc::new(RefCell::new(element)),
            links: Vec::new()
        })
    }

    pub fn push(&mut self, mut linkable: Box<Linkable<E>>) {
        linkable.set_element_link(self.element.clone());
        self.links.push(linkable);
    }
}
impl<E> Element for Link<E> where E: Element {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        (*self.element.borrow_mut()).setup(ui);;
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool {
        self.is_setup && (*self.element.borrow()).is_setup()
    }

    fn stop(&self) {
        (*self.element.borrow_mut()).stop();
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {
        (*self.element.borrow_mut()).build_window(ui);
    }

    fn get_frame(&self) -> Frame<i32> {
        (*self.element.borrow()).get_frame()
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        (*self.element.borrow_mut()).set_frame(frame, window_center);
    }

    fn get_min_size(&self) -> Vec2<i32> {
        (*self.element.borrow()).get_min_size()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        (*self.element.borrow()).get_max_size()
    }
    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) {
        for link in &mut self.links {
            link.transmit_msg(msg.clone(), true);
        }
        if !stop {
            (*self.element.borrow_mut()).transmit_msg(msg, false);
        }
    }
}














pub struct Animation<E: Element> {
    is_setup: bool,
    element: Option<Box<E>>,
    element_link: Option<Rc<RefCell<Box<E>>>>,


}





































pub struct Socket<E: Element> {
    is_setup: bool,
    element: Option<Box<E>>,
    receive: Box<Fn(&mut E, ActionMsg)>,

    element_link: Option<Rc<RefCell<Box<E>>>>,
}
impl<E> Socket<E> where E: Element {

    pub fn new() -> Box<Self> {
        Box::new(Socket {
            is_setup: false,
            element: None,
            receive: Box::new(|_,_|{}),
            element_link: None,
        })
    }

    pub fn with_element(mut self, element: Box<E>) -> Box<Self> {
        self.element = Some(element);
        Box::new(self)
    }

    pub fn with_action_receive(mut self, fun: Box<Fn(&mut E, ActionMsg)>) -> Box<Self> {
        self.receive = fun;
        Box::new(self)
    }
}

impl<E> Linkable<E> for Socket<E> where E: Element {
    fn set_element_link(&mut self, element: Rc<RefCell<Box<E>>>) {
        self.element_link = Some(element);
    }
}

impl<E> Element for Socket<E> where E: Element {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        if let Some(ref mut el) = self.element {
            el.setup(ui);
        }
        if let Some(ref el) = self.element_link {
            (*el.borrow_mut()).setup(ui);
        }
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool {
        let b0 = if let Some(ref el) = self.element {
            el.is_setup()
        } else { true };

        let b1 = if let Some(ref el) = self.element_link {
            (*el.borrow()).is_setup()
        } else { true };

        self.is_setup && b0 && b1
    }

    fn stop(&self) {
        if let Some(ref el) = self.element {
            el.stop();
        }
        if let Some(ref el) = self.element_link {
            (*el.borrow()).stop();
        }
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {
        if let Some(ref el) = self.element {
            el.build_window(ui);
        }
        if let Some(ref el) = self.element_link {
            (*el.borrow()).build_window(ui);
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        if let Some(ref el) = self.element {
            return el.get_frame();
        }
        if let Some(ref el) = self.element_link {
            return (*el.borrow()).get_frame();
        }
        Frame::new()
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        if let Some(ref mut el) = self.element {
            el.set_frame(frame, window_center);
        }
        if let Some(ref el) = self.element_link {
            (*el.borrow_mut()).set_frame(frame, window_center);
        }
    }

    fn get_min_size(&self) -> Vec2<i32> {
        if let Some(ref el) = self.element {
            return el.get_min_size();
        }
        if let Some(ref el) = self.element_link {
            return (*el.borrow()).get_min_size();
        }
        Vec2::zero()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        if let Some(ref el) = self.element {
            return el.get_max_size();
        }
        if let Some(ref el) = self.element_link {
            return (*el.borrow()).get_max_size();
        }
        Vec2 {
            x: i32::MAX, y: i32::MAX
        }
    }
    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) {
        // first socket, then content
        if let Some(ref mut el) = self.element {
            (self.receive)(el, msg.clone());
            if !stop { el.transmit_msg(msg.clone(), false); }
        }
        if let Some(ref el) = self.element_link {
            (self.receive)(&mut *el.borrow_mut(), msg.clone());
            if !stop { (&mut *el.borrow_mut()).transmit_msg(msg, false); }
        }
    }
}



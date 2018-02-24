
use conrod;

use elements::*;


use time::precise_time_ns;



#[allow(dead_code)]
const DEBUG: bool = false;












/*
 .d8b.   .o88b. d888888b d888888b  .d88b.  d8b   db .88b  d88. .d8888.  d888b
d8' `8b d8P  Y8 `~~88~~'   `88'   .8P  Y8. 888o  88 88'YbdP`88 88'  YP 88' Y8b
88ooo88 8P         88       88    88    88 88V8o 88 88  88  88 `8bo.   88
88~~~88 8b         88       88    88    88 88 V8o88 88  88  88   `Y8b. 88  ooo
88   88 Y8b  d8    88      .88.   `8b  d8' 88  V888 88  88  88 db   8D 88. ~8~
YP   YP  `Y88P'    YP    Y888888P  `Y88P'  VP   V8P YP  YP  YP `8888Y'  Y888P


*/




use std::sync::mpsc::{Sender};





#[derive(Debug, Clone, PartialEq)]
pub enum ActionMsgData {
    Mouse(f64,f64),
    MousePressLeft(f64,f64),
    MousePressRight(f64,f64),
    MousePressMiddle(f64,f64),
    MouseReleaseLeft(f64,f64),
    MouseReleaseRight(f64,f64),
    MouseReleaseMiddle(f64,f64),
    MouseDragLeft(f64,f64),
    MouseDragRight(f64,f64),
    MouseDragMiddle(f64,f64),
    MouseWheel(f64, f64), // TODO implement
    Click,
    Press,
    Release,
    Hover,
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






















/*
 .d8b.  d8b   db d888888b .88b  d88.  .d8b.  d888888b d888888b  .d88b.  d8b   db
d8' `8b 888o  88   `88'   88'YbdP`88 d8' `8b `~~88~~'   `88'   .8P  Y8. 888o  88
88ooo88 88V8o 88    88    88  88  88 88ooo88    88       88    88    88 88V8o 88
88~~~88 88 V8o88    88    88  88  88 88~~~88    88       88    88    88 88 V8o88
88   88 88  V888   .88.   88  88  88 88   88    88      .88.   `8b  d8' 88  V888
YP   YP VP   V8P Y888888P YP  YP  YP YP   YP    YP    Y888888P  `Y88P'  VP   V8P


*/


widget_ids!(
    #[derive(Clone)]
    struct AnimationIds {
        animation,
    }
);

pub trait Animateable : Element {
    fn animate_size(&mut self, _xy: (Dim,Dim)) {}
    fn animate_position(&mut self, _xy: (Dim,Dim)) {}
    fn start(&mut self){}
    fn run(&mut self){}
    fn reset(&mut self);
}



pub trait SizeAnimation {
    fn calc(&self, t: f64, duration: f64) -> (Dim, Dim);
}

pub trait PositionAnimation {
    fn calc(&self, t: f64, duration: f64) -> (Dim, Dim);
}






pub struct Animation {
    ids: Option<AnimationIds>,
    parent: Option<conrod::widget::id::Id>,
    floating: bool,

    pub element: Box<Animateable>,

    size_animation: Option<Box<SizeAnimation>>,
    position_animation: Option<Box<PositionAnimation>>,
    min_size: Vec2<i32>,
    max_size: Vec2<i32>,

    duration: f64,
    start_time: u64,
    running: bool,
}

impl Animation {
    pub fn new(element: Box<Animateable>) -> Box<Self> {
        Box::new(Animation {
            ids: None,
            parent: None,
            floating: false,
            element,
            size_animation: None,
            position_animation: None,
            min_size: Vec2::zero(),
            max_size: Vec2 {x: i32::MAX, y: i32::MAX},
            duration: 100.0,
            start_time: 0,
            running: false
        })
    }

    pub fn with_duration(mut self, duration_ms: f64) -> Box<Self> {
        self.duration = duration_ms;
        Box::new(self)
    }

    pub fn with_size_animation(mut self, animation: Box<SizeAnimation>) -> Box<Self> {
        self.size_animation = Some(animation);
        Box::new(self)
    }

    pub fn with_position_animation(mut self, animation: Box<PositionAnimation>) -> Box<Self> {
        self.position_animation = Some(animation);
        Box::new(self)
    }

    pub fn with_floating(mut self, floating: bool) -> Box<Self> {
        self.floating = floating;
        Box::new(self)
    }

    fn time(&self) -> f64 {
        (precise_time_ns() - self.start_time) as f64 * 1e-6
    }
}


impl Animateable for Animation {
    fn animate_size(&mut self, xy: (Dim,Dim)) {
        self.element.animate_size(xy);
    }
    fn animate_position(&mut self, xy: (Dim,Dim)) {
        self.element.animate_position(xy);
    }

    fn start(&mut self) {
        // start animation if not already running
        if !self.running {
            self.start_time = precise_time_ns();
            self.running = true;
        }
        // TODO not ideal. need to manage individual floating property
        self.element.set_floating(self.floating);
        self.element.start();
    }

    fn run(&mut self) {
        let mut do_reset = false;
        let tau = self.duration;
        let t = self.time();
        let t = if self.running && t < self.duration {
            t
        } else {
            self.running = false;
            do_reset = true;
            tau
        };

        if let Some(ref f) = self.size_animation {
            self.element.animate_size(f.calc(t,tau));
        }
        if let Some(ref f) = self.position_animation {
            self.element.animate_position(f.calc(t,tau));
        }

        if do_reset { self.element.reset() }
    }

    fn reset(&mut self) {
        if !self.running {
            // TODO not ideal. need to manage individual floating property
            self.element.set_floating(false);
            self.element.reset();
        }
    }
}


impl Element for Animation {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.ids = Some(AnimationIds::new(ui.widget_id_generator()));
        self.element.setup(ui);
    }
    fn is_setup(&self) -> bool {
        self.element.is_setup()
    }

    fn set_floating(&mut self, floating: bool) {
        self.element.set_floating(floating);
    }

    fn set_parent_widget(&mut self, parent: conrod::widget::id::Id) {
        self.parent = Some(parent);
        self.element.set_parent_widget(parent);
    }

    fn stop(&mut self) {
        self.element.stop();
        self.running = false;
    }
    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        self.element.build_window(ui, ressources);
    }

    fn get_frame(&self) -> Frame<i32> {
        self.element.get_frame()
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.element.set_frame(frame, window_center);
    }

    fn set_max_size(&mut self, size: Vec2<i32>) {
        self.max_size = size;
    }
    fn set_min_size(&mut self, size: Vec2<i32>) {
        self.min_size = size;
    }

    fn get_min_size(&self) -> Vec2<i32> {
        self.element.get_min_size()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        self.element.get_max_size()
    }
    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) {
        match msg.msg {
            ActionMsgData::Update => {
                self.run();
            },
            _ => ()
        }

        if !stop {
            self.element.transmit_msg(msg, false);
        }
    }
}































/*
.d8888.  .d88b.   .o88b. db   dD d88888b d888888b
88'  YP .8P  Y8. d8P  Y8 88 ,8P' 88'     `~~88~~'
`8bo.   88    88 8P      88,8P   88ooooo    88
  `Y8b. 88    88 8b      88`8b   88~~~~~    88
db   8D `8b  d8' Y8b  d8 88 `88. 88.        88
`8888Y'  `Y88P'   `Y88P' YP   YD Y88888P    YP


*/




widget_ids!(
    #[derive(Clone)]
    struct SocketIds {
        socket,
    }
);

pub struct Socket<E: Element> {
    ids: Option<SocketIds>,
    parent: Option<conrod::widget::id::Id>,

    min_size: Vec2<i32>,
    max_size: Vec2<i32>,

    is_setup: bool,
    element: Box<E>,
    receive: Box<Fn(&mut E, ActionMsg)>,
}
impl<E> Socket<E> where E: Element {

    pub fn new(element: Box<E>) -> Box<Self> {
        Box::new(Socket {
            ids: None,
            parent: None,
            min_size: Vec2::zero(),
            max_size: Vec2 {x: i32::MAX, y: i32::MAX},
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
        let ids = SocketIds::new(ui.widget_id_generator());
        if let Some(parent) = self.parent {
            self.element.set_parent_widget(parent);
        }
        self.element.setup(ui);
        self.ids = Some(ids);
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool {
        self.is_setup && self.element.is_setup()
    }

    fn set_parent_widget(&mut self, parent: conrod::widget::id::Id) {
        self.parent = Some(parent);
        self.element.set_parent_widget(parent);
    }

    fn set_floating(&mut self, floating: bool) {
        self.element.set_floating(floating);
    }

    fn stop(&mut self) {
        self.element.stop();
    }
    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        self.element.build_window(ui, ressources);
    }

    fn get_frame(&self) -> Frame<i32> {
        self.element.get_frame()
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.element.set_frame(frame, window_center);
    }

    fn set_max_size(&mut self, size: Vec2<i32>) {
        self.max_size = size;
    }
    fn set_min_size(&mut self, size: Vec2<i32>) {
        self.min_size = size;
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



use conrod;
use elements::{*, action::*, basic::*, shared::*, structures::*};
use std::sync::mpsc::Sender;
use std::cell::RefCell;
use std::rc::Rc;
use std::i32;



const DEBUG: bool = false;





/*
d8888b. db    db d888888b d888888b  .d88b.  d8b   db
88  `8D 88    88 `~~88~~' `~~88~~' .8P  Y8. 888o  88
88oooY' 88    88    88       88    88    88 88V8o 88
88~~~b. 88    88    88       88    88    88 88 V8o88
88   8D 88b  d88    88       88    `8b  d8' 88  V888
Y8888P' ~Y8888P'    YP       YP     `Y88P'  VP   V8P


*/


widget_ids!(
    #[derive(Clone)]
    struct ButtonIds {
        button,
    }
);



#[derive(Clone)]
pub struct Button {
    id: String,
    senders: Vec<Sender<ActionMsg>>,

    //receive_fn: Box<Fn(&mut Element, ActionMsg)>,

    is_setup: bool,
    global_center: Vec2<i32>,
    frame: Frame<i32>,
    min_size: Vec2<i32>,
    max_size: Vec2<i32>,

    button_ids: Option<ButtonIds>,
    parent: Option<conrod::widget::id::Id>,
    floating: bool,
    
    plane: Box<Plane>,
    plane_hover: Box<Plane>,
    plane_click: Box<Plane>,
    is_hover: bool,
    is_click: bool,

    label: Option<Box<Text>>,
}

impl Button {
    pub fn new() -> Box<Self> {
        Box::new(Button {
            id: "Button".to_string(),
            senders: Vec::new(),
            //receive_fn: rfun,

            is_setup: false,
            global_center: Vec2::zero(),
            frame: Frame::new(),
            min_size: Vec2::zero(),
            max_size: Vec2 {x: i32::MAX, y: i32::MAX},

            button_ids: None,
            parent: None,
            floating: false,

            plane: Plane::new(Graphic::Color(conrod::color::LIGHT_GREY)),
            plane_hover: Plane::new(Graphic::Color(conrod::color::LIGHT_YELLOW)),
            plane_click: Plane::new(Graphic::Color(conrod::color::LIGHT_GREEN)),
            is_hover: false,
            is_click: false,

            label: None
        })
    }

    pub fn with_graphics(mut self, std: Graphic, hover: Graphic, click: Graphic) -> Box<Self> {
        self.plane = Plane::new(std);
        self.plane_hover = Plane::new(hover);
        self.plane_click = Plane::new(click);
        Box::new(self)
    }

    pub fn with_graphic(mut self, std: Graphic) -> Box<Self> {
        self.plane = Plane::new(std);
        Box::new(self)
    }

    pub fn with_graphic_hover(mut self, hover: Graphic) -> Box<Self> {
        self.plane_hover = Plane::new(hover);
        Box::new(self)
    }

    pub fn with_graphic_click(mut self, click: Graphic) -> Box<Self> {
        self.plane_click = Plane::new(click);
        Box::new(self)
    }
}

impl Labelable for Button {
    fn with_font(mut self, font: Font) -> Box<Self> {
        self.label = Some(Text::new(font));
        Box::new(self)
    }
    fn set_font(&mut self, font: Font) {
        self.label = Some(Text::new(font));
    }
}


impl ActionSendable for Button {
    fn with_id(mut self, id: String) -> Box<Self> {
        self.id = id;
        Box::new(self)
    }
    fn with_sender(mut self, sender: Sender<ActionMsg>) -> Box<Self> {
        self.senders.push(sender);
        Box::new(self)
    }
}



impl Element for Button {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        let ids = ButtonIds::new(ui.widget_id_generator());

        self.plane.set_parent_widget(ids.button);
        self.plane.setup(ui);
        self.plane_hover.set_parent_widget(ids.button);
        self.plane_hover.setup(ui);
        self.plane_click.set_parent_widget(ids.button);
        self.plane_click.setup(ui);

        if let Some(ref mut label) = self.label {
            label.set_parent_widget(ids.button);
            label.setup(ui);
        }

        self.button_ids = Some(ids);
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool {
        let mut label_setup = true;
        if let Some(ref label) = self.label {
            label_setup = label.is_setup();
        }
        self.is_setup && label_setup
            && self.plane.is_setup()
            && self.plane_click.is_setup()
            && self.plane_hover.is_setup()
    }

    fn set_parent_widget(&mut self, parent: conrod::widget::id::Id) {
        self.parent = Some(parent);
    }

    fn set_floating(&mut self, floating: bool) {
        self.floating = floating;
    }

    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        use conrod::{widget, Widget, Colorable, Borderable};

        if let Some(ref ids) = self.button_ids {
            let mut canvas = widget::canvas::Canvas::new()
                .border(0f64)
                .rgba(0.0,0.0,0.0,0.0)
                .floating(self.floating);
            if let Some(parent) = self.parent {
                canvas = canvas.parent(parent);
            }
            canvas.set(ids.button, ui);
        }
        
        if self.is_click {
            self.plane_click.build_window(ui, ressources);
        } else if self.is_hover {
            self.plane_hover.build_window(ui, ressources);
        } else {
            self.plane.build_window(ui, ressources);
        }

        if let Some(ref label) = self.label {
            label.build_window(ui,ressources);
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.global_center = window_center;
        self.frame = frame;

        self.plane.set_frame(frame, window_center);
        self.plane_hover.set_frame(frame, window_center);
        self.plane_click.set_frame(frame, window_center);

        if let Some(ref mut label) = self.label {
            label.set_frame(frame, window_center);
        }
    }

    fn set_min_size(&mut self, size: Vec2<i32>) {
        self.min_size = size;
    }
    fn get_min_size(&self) -> Vec2<i32> {
        self.min_size
    }
    fn set_max_size(&mut self, size: Vec2<i32>) {
        self.max_size = size;
    }
    fn get_max_size(&self) -> Vec2<i32> {
        self.max_size
    }

    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) -> Option<ActionMsg> {
        let mut used_up = false;
        let id = msg.sender_id.clone();
        match msg.msg {
            ActionMsgData::MouseGone => {
                self.is_hover = false;
                if self.is_click {
                    self.is_click = false;
                    for sender in &self.senders {
                        let _ = sender.send(ActionMsg{
                            sender_id: self.id.clone(),
                            msg: ActionMsgData::Release
                        });
                    }
                }
            },
            ActionMsgData::Mouse(x,y) => {
                if self.frame.inside(x as i32,y as i32) {
                    for sender in &self.senders {
                        let _ = sender.send(ActionMsg{
                            sender_id: self.id.clone(),
                            msg: ActionMsgData::Hover
                        });
                    }
                    self.is_hover = true;
                    used_up = true;
                } else {
                    if self.is_click {
                        for sender in &self.senders {
                            let _ = sender.send(ActionMsg{
                                sender_id: self.id.clone(),
                                msg: ActionMsgData::Release
                            });
                        }
                    }
                    if self.is_hover {
                        for sender in &self.senders {
                            let _ = sender.send(ActionMsg{
                                sender_id: self.id.clone(),
                                msg: ActionMsgData::HoverGone
                            });
                        }
                        self.is_hover = false;
                    }
                    self.is_click = false;
                }
            },
            ActionMsgData::MousePressLeft(x,y) => {
                if self.frame.inside(x as i32, y as i32) {
                    for sender in &self.senders {
                        let _ = sender.send(ActionMsg{
                            sender_id: self.id.clone(),
                            msg: ActionMsgData::Press
                        });
                    }
                    self.is_click = true;
                    used_up = true;
                }
            },
            ActionMsgData::MouseReleaseLeft(_,_) => {
                if self.is_click {
                    for sender in &self.senders {
                        let _ = sender.send(ActionMsg{
                            sender_id: self.id.clone(),
                            msg: ActionMsgData::Click
                        });
                        let _ = sender.send(ActionMsg{
                            sender_id: self.id.clone(),
                            msg: ActionMsgData::Release
                        });
                    }
                }
                self.is_click = false;
            },
            _ => (),
        }
        if used_up {
            let _ = self.plane.transmit_msg(msg.clone(), stop);
            let _ = self.plane_hover.transmit_msg(msg.clone(), stop);
            let _ = self.plane_click.transmit_msg(msg, stop);
            Some(ActionMsg{
                sender_id: self.id.clone(),
                msg: ActionMsgData::MouseGone
            })
        } else {
            Some(msg)
        }
    }
}









/*
.d8888.  .o88b. d8888b.  .d88b.  db      db
88'  YP d8P  Y8 88  `8D .8P  Y8. 88      88
`8bo.   8P      88oobY' 88    88 88      88
  `Y8b. 8b      88`8b   88    88 88      88
db   8D Y8b  d8 88 `88. `8b  d8' 88booo. 88booo.
`8888Y'  `Y88P' 88   YD  `Y88P'  Y88888P Y88888P


*/





widget_ids!(
    struct ScrollIds {
        scroll
    }
);

#[derive(Debug, Copy, Clone)]
pub enum ScrollAlignment {
    Horizontal,
    Vertical,
}


pub struct Scroll {
    id: String,
    ids: Option<ScrollIds>,
    parent: Option<conrod::widget::id::Id>,

    elements: Vec<Box<Element>>,
    alignment: ScrollAlignment,
    scroll_bar: Box<Socket<Button>>,
    scroll_bar_width: i32,
    scroll_position: Rc<RefCell<(f64,f64)>>,
    scroll_trigger: Rc<RefCell<bool>>,

    is_setup: bool,
    frame: Frame<i32>,
    global_center: Vec2<i32>,
    min_size: Vec2<i32>,
    max_size: Vec2<i32>,
}

impl Scroll {

    pub fn new(alignment: ScrollAlignment, id: String, sender: Sender<ActionMsg>) -> Box<Self> {
        Self::new_with_button(
            id.clone(),
            Button::new()
                .with_graphic(Graphic::Color(conrod::color::LIGHT_BLUE))
                .with_id(id)
                .with_sender(sender),
            alignment
        )
    }

    pub fn new_with_button(id: String, scrollbar_button: Box<Button>, alignment: ScrollAlignment) -> Box<Self> {
        use std::i32;

        let scroll_trigger = Rc::new(RefCell::new(false));
        let scroll_position = Rc::new(RefCell::new( (0.0,0.0) ));

        let scrollp = scroll_position.clone();
        let scrolltr = scroll_trigger.clone();
        let scroll_bar = Socket::new(scrollbar_button)
            .with_action_receive(Box::new(move |button, msg|{
            match msg.msg {
                ActionMsgData::MousePressLeft(x,y) => {
                    if button.get_frame().inside(x as i32, y as i32) {
                        (*scrolltr.borrow_mut()) = true;
                        return None;
                    }
                },
                ActionMsgData::MouseDragLeft(x,y) => {
                    if *scrolltr.borrow() {
                        match alignment {
                            ScrollAlignment::Horizontal => {
                                (*scrollp.borrow_mut()).1 = x;
                            },
                            ScrollAlignment::Vertical => {
                                (*scrollp.borrow_mut()).1 = y;
                            }
                        }
                        return None;
                    }
                    
                },
                ActionMsgData::MouseReleaseLeft(_,_) => {
                    let triggered = *scrolltr.borrow();
                    if triggered {
                        let delta = (*scrollp.borrow()).1;
                        (*scrollp.borrow_mut()).0 += delta;
                        (*scrollp.borrow_mut()).1 = 0.0;
                        (*scrolltr.borrow_mut()) = false;
                    }
                }
                _ => ()
            }
            Some(msg)

        }));

        Box::new(Scroll {
            id,
            ids: None,
            parent: None,
            elements: Vec::new(),
            alignment,
            scroll_bar,
            scroll_bar_width: 15,
            scroll_position,
            scroll_trigger,
            is_setup: false,
            frame: Frame::new(),
            global_center: Vec2::zero(),
            min_size: Vec2::zero(),
            max_size: Vec2 {x: i32::MAX, y: i32::MAX},
        })
    }

    pub fn push(&mut self, element: Box<Element>) {
        let n = self.elements.len();
        self.insert(n, element);
    }

    pub fn insert(&mut self, index: usize, element: Box<Element>) {

        if index >= self.elements.len() {
            self.elements.push(element);
        } else {
            self.elements.insert(index, element);
        }

        self.rescale_elements();
        self.is_setup = false;
    }

    pub fn pop(&mut self) -> Option<Box<Element>> {
        let el = self.elements.pop();
        self.rescale_elements();
        el
    }

    pub fn remove(&mut self, index: usize) -> Box<Element> {
        let el = self.elements.remove(index);
        self.rescale_elements();
        el
    }


    fn get_elements_min_size(&self) -> Vec2<i32> {
        let mut min = Vec2::zero();
        for el in &self.elements {
            let tmp = el.get_min_size();
            match self.alignment {
                ScrollAlignment::Horizontal => {
                    min.x += tmp.x;
                    if min.y < tmp.y { min.y = tmp.y; }
                },
                ScrollAlignment::Vertical   => {
                    min.y += tmp.y;
                    if min.x < tmp.x { min.x = tmp.x; }
                },
            }
        }
        min
    }
    fn get_elements_max_size(&self) -> Vec2<i32> {
        use std::i32;
        let mut max = Vec2::zero();
        for el in &self.elements {
            let tmp = el.get_max_size();
            match self.alignment {
                ScrollAlignment::Horizontal => {
                    max.x = if tmp.x == i32::MAX {
                        i32::MAX
                    } else {
                        max.x + tmp.x
                    };
                    if max.y > tmp.y { max.y = tmp.y; }
                },
                ScrollAlignment::Vertical   => {
                    max.y = if tmp.y == i32::MAX {
                        i32::MAX
                    } else {
                        max.y + tmp.y
                    };
                    if max.x < tmp.x { max.x = tmp.x; }
                },
            }
        }
        max
    }

    fn is_inside_area(&self) -> bool {
        let min = self.get_elements_min_size();

        match self.alignment {
            ScrollAlignment::Horizontal => {
                self.frame.size().x > min.x
            },
            ScrollAlignment::Vertical => {
                self.frame.size().y > min.y
            }
        }
    }


    fn rescale_elements(&mut self) {
        if DEBUG { println!("rescaling...");}

        let n = self.elements.len();
        let s = self.frame.size();

        let elmin = self.get_elements_min_size();

        let (s0,s1) = *self.scroll_position.borrow();
        let mut sp = s0 + s1;

        match self.alignment {
            ScrollAlignment::Horizontal => {
                let frac = s.x as f64/elmin.x as f64;
                let bar = (frac * s.x as f64) as i32;

                if sp < 0.0 {
                    sp = 0.0;
                    //(*self.scroll_position.borrow_mut()) = (0.0,0.0);
                }
                if sp > (1.0-frac) * s.y as f64 {
                    sp = (1.0-frac) * s.y as f64;
                    //(*self.scroll_position.borrow_mut()) = (sp,0.0);
                }

                let scroll = (sp as f64)/(s.x as f64);
                let delta = (scroll * self.get_elements_min_size().x as f64) as i32;

                let bar_shift = if self.is_inside_area() {0} else {self.scroll_bar_width};

                let mut xp = 0;
                for ix in 0..n {
                    let el = &mut self.elements[ix];
                    let min = el.get_min_size().x;
                    el.set_frame(Frame{
                        p0: Vec2{x: delta + xp + self.frame.p0.x, y: self.frame.p0.y + bar_shift},
                        p1: Vec2{x: delta + xp + min + self.frame.p0.x, y: self.frame.p1.y}
                    }, self.global_center);
                    xp += min;
                }

                // scrollbar
                self.scroll_bar.set_frame(
                    Frame{
                        p0: Vec2 {
                            y: self.frame.p0.y,
                            x: (scroll * s.x as f64) as i32 + self.frame.p0.x
                        },
                        p1: Vec2 {
                            y: self.frame.p0.y + self.scroll_bar_width,
                            x: (scroll * s.x as f64) as i32 + self.frame.p0.x + bar,
                        }
                    },
                    self.global_center
                );
            },
            ScrollAlignment::Vertical => {
                let frac = s.y as f64/elmin.y as f64;
                let bar = (frac * s.y as f64) as i32;

                if sp > 0.0 {
                    sp = 0.0;
                    //(*self.scroll_position.borrow_mut()) = (0.0,0.0);
                }
                if sp < -(1.0-frac) * s.y as f64 {
                    sp = -(1.0-frac) * s.y as f64;
                    //(*self.scroll_position.borrow_mut()) = (sp,0.0);
                }

                let scroll = (sp as f64)/(s.y as f64);
                let delta = -(scroll * self.get_elements_min_size().y as f64) as i32;


                let bar_shift = if self.is_inside_area() {0} else {self.scroll_bar_width};


                let mut yp = 0;
                for ix in 0..n {
                    let el = &mut self.elements[ix];
                    let min = el.get_min_size().y;
                    el.set_frame(Frame{
                        p0: Vec2{x: self.frame.p0.x, y: delta + self.frame.p1.y - yp - min},
                        p1: Vec2{x: self.frame.p1.x - bar_shift, y: delta + self.frame.p1.y - yp}
                    }, self.global_center);
                    yp += min;
                }

                // scrollbar
                self.scroll_bar.set_frame(
                    Frame{
                        p0: Vec2 {
                            x: self.frame.p1.x - self.scroll_bar_width,
                            y: (scroll * s.y as f64) as i32 + self.frame.p1.y - bar
                        },
                        p1: Vec2 {
                            x: self.frame.p1.x,
                            y: (scroll * s.y as f64) as i32 + self.frame.p1.y,
                        }
                    },
                    self.global_center
                );
            },
        }
        if DEBUG { println!("... rescaling done.");}
    }
}

impl Element for Scroll {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        // elements

        let ids = ScrollIds::new(ui.widget_id_generator());
        for el in &mut self.elements {
            if !el.is_setup() { 
                el.set_parent_widget(ids.scroll);
                el.setup(ui); 
            }
        }

        // scroll bar
        self.scroll_bar.set_parent_widget(ids.scroll);
        self.scroll_bar.setup(ui);

        self.ids = Some(ids);
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool {
        let mut setup = self.is_setup;
        for el in &self.elements {
            if !el.is_setup() { setup = false; }
        }
        if !self.scroll_bar.is_setup() { setup = false; }
        if DEBUG { println!("Scroll is setup? {}", setup); }
        setup
    }

    fn set_parent_widget(&mut self, parent: conrod::widget::id::Id) {
        self.parent = Some(parent);
    }
    // TODO maybe implement?
    fn set_floating(&mut self, _floating: bool){}

    fn stop(&mut self) {
        for el in &mut self.elements {
            el.stop();
        }
    }
    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        use conrod::{widget, Widget, Positionable};

        if let Some(ref ids) = self.ids {
            let frame = self.frame;
            let c = frame.center() - self.global_center;

            let mut rect = widget::Rectangle::fill_with(
                [frame.width() as f64, frame.height() as f64], conrod::color::Color::Rgba(0.0,0.0,0.0,0.0)
            ).x_y(c.x as f64, c.y as f64).crop_kids();
            if let Some(parent) = self.parent {
                rect = rect.parent(parent);
            }
            rect.set(ids.scroll, ui);
        }
        
        for el in &self.elements {
            el.build_window(ui, ressources);
        }

        if !self.is_inside_area() {
            self.scroll_bar.build_window(ui, ressources);
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.global_center = window_center;
        self.frame = frame;
        self.rescale_elements();
    }

    fn set_min_size(&mut self, size: Vec2<i32>) {
        self.min_size = size;
    }
    fn set_max_size(&mut self, size: Vec2<i32>) {
        self.max_size = size;
    }

    fn get_min_size(&self) -> Vec2<i32> {
        let min = self.get_elements_min_size();
        let x = if min.x < self.min_size.x {min.x} else {self.min_size.x};
        let y = if min.y < self.min_size.y {min.y} else {self.min_size.y};
        Vec2{x, y}
    }
    fn get_max_size(&self) -> Vec2<i32> {
        let max = self.get_elements_max_size();

        let x;
        let y;
        match self.alignment {
            ScrollAlignment::Horizontal => {
                x = self.max_size.x;
                y = if max.y < self.max_size.y {max.y} else {self.max_size.y};
            },
            ScrollAlignment::Vertical => {
                y = self.max_size.y;
                x = if max.x < self.max_size.x {max.x} else {self.max_size.x};
            }
        }
        Vec2{x,y}
    }

    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) -> Option<ActionMsg> {
        let mut loc_msg = self.scroll_bar.transmit_msg(msg.clone(), false);
        if *self.scroll_trigger.borrow() {
            self.rescale_elements();
        }
        if !stop && !*self.scroll_trigger.borrow(){
            let id = self.id.clone();
            match msg.msg {
                ActionMsgData::Mouse(x,y)
                | ActionMsgData::MousePressLeft(x,y)
                | ActionMsgData::MousePressRight(x,y)
                | ActionMsgData::MousePressMiddle(x,y) => {
                    if self.frame.inside(x as i32, y as i32) {
                        for el in &mut self.elements {
                            if let Some(tmp) = loc_msg.clone() {
                                el.transmit_msg(tmp, false);
                            }
                        }
                    } else {
                        for el in &mut self.elements {
                            el.transmit_msg(ActionMsg{
                                sender_id: id.clone(),
                                msg: ActionMsgData::MouseGone
                            }, false);
                        }
                    }
                },
                _ => {
                    for el in &mut self.elements {
                        if let Some(tmp) = loc_msg {
                            loc_msg = el.transmit_msg(tmp, false);
                        }
                    }
                }
            }
        }
        loc_msg
    }
}

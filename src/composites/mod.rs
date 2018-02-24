

use conrod;
use elements::{*, action::*, basic::*, shared::*, structures::*};
use std::sync::mpsc::Sender;
use std::cell::RefCell;
use std::rc::Rc;



const DEBUG: bool = false;







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
        Self::new_with_graphic(
            Graphic::Color(conrod::color::LIGHT_BLUE), 
            alignment, id, sender
        )
    }

    pub fn new_with_graphic(scrollbar_graphic: Graphic, alignment: ScrollAlignment, id: String, sender: Sender<ActionMsg>) -> Box<Self> {
        use std::i32;

        let scroll_trigger = Rc::new(RefCell::new(false));
        let scroll_position = Rc::new(RefCell::new( (0.0,0.0) ));

        let scrollp = scroll_position.clone();
        let scrolltr = scroll_trigger.clone();
        let scroll_bar = Socket::new(
            Button::new()
                .with_graphic(scrollbar_graphic)
                .with_id(id)
                .with_sender(sender)
        ).with_action_receive(Box::new(move |button, msg|{
            match msg.msg {
                ActionMsgData::MousePressLeft(x,y) => {
                    if button.get_frame().inside(x as i32, y as i32) {
                        println!("scroll pressed: {:?}", msg);
                        (*scrolltr.borrow_mut()) = true;
                    }
                },
                ActionMsgData::MouseDragLeft(x,y) => {
                    if *scrolltr.borrow() {
                        let (s0,s1) = *scrollp.borrow();
                        println!("scroll {} + {} dragged {}, {}",s0,s1,x,y);
                        match alignment {
                            ScrollAlignment::Horizontal => {
                                (*scrollp.borrow_mut()).1 = x;
                            },
                            ScrollAlignment::Vertical => {
                                (*scrollp.borrow_mut()).1 = y;
                            }
                        }
                    }
                    
                },
                ActionMsgData::MouseReleaseLeft(_,_) => {
                    let triggered = *scrolltr.borrow();
                    if triggered {
                        let delta = (*scrollp.borrow()).1;
                        println!("scroll dragging stopped. adding {}", delta);
                        (*scrollp.borrow_mut()).0 += delta;
                        (*scrollp.borrow_mut()).1 = 0.0;
                        (*scrolltr.borrow_mut()) = false;
                    }
                }
                _ => ()
            }
        }));

        Box::new(Scroll {
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
                    (*self.scroll_position.borrow_mut()) = (0.0,0.0);
                }
                if sp > (1.0-frac) * s.y as f64 {
                    sp = (1.0-frac) * s.y as f64;
                    (*self.scroll_position.borrow_mut()) = (sp,0.0);
                }

                let scroll = (sp as f64)/(s.x as f64);
                let delta = (scroll * self.get_elements_min_size().x as f64) as i32;

                let mut xp = 0;
                for ix in 0..n {
                    let el = &mut self.elements[ix];
                    let min = el.get_min_size().x;
                    el.set_frame(Frame{
                        p0: Vec2{x: delta + xp + self.frame.p0.x, y: self.frame.p0.y},
                        p1: Vec2{x: delta + xp + min + self.frame.p0.x, y: self.frame.p1.y}
                    }, self.global_center);
                    xp += min;
                }

                // scrollbar
                self.scroll_bar.set_frame(
                    Frame{
                        p0: Vec2 {
                            y: self.frame.p0.y + self.scroll_bar_width,
                            x: (scroll * s.x as f64) as i32 + self.frame.p0.x
                        },
                        p1: Vec2 {
                            y: self.frame.p0.y,
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
                    (*self.scroll_position.borrow_mut()) = (0.0,0.0);
                }
                if sp < -(1.0-frac) * s.y as f64 {
                    sp = -(1.0-frac) * s.y as f64;
                    (*self.scroll_position.borrow_mut()) = (sp,0.0);
                }

                let scroll = (sp as f64)/(s.y as f64);
                let delta = -(scroll * self.get_elements_min_size().y as f64) as i32;

                let mut yp = 0;
                for ix in 0..n {
                    let el = &mut self.elements[ix];
                    let min = el.get_min_size().y;
                    el.set_frame(Frame{
                        p0: Vec2{x: self.frame.p0.x, y: delta + self.frame.p1.y - yp - min},
                        p1: Vec2{x: self.frame.p1.x, y: delta + self.frame.p1.y - yp}
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

    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) {
        self.scroll_bar.transmit_msg(msg.clone(), false);
        if *self.scroll_trigger.borrow() {
            self.rescale_elements();
        }
        if !stop {
            for el in &mut self.elements {
                el.transmit_msg(msg.clone(), false);
            }
        }
    }
}

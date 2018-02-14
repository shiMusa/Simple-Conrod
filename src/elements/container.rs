


use conrod;

use elements::*;










const DEBUG: bool = false;












pub struct Empty {
    frame: Frame<i32>
}
impl Empty {
    pub fn new() -> Box<Self> {
        Box::new(Empty{frame: Frame::new()})
    }
}
impl Element for Empty {
    fn setup(&mut self, _ui: &mut conrod::Ui) {}
    fn build_window(&self, _ui: &mut conrod::UiCell) {}

    fn get_frame(&self) -> Frame<i32> { self.frame }
    fn set_frame(&mut self, frame: Frame<i32>) {
        self.frame = frame;
    }

    fn set_window_center(&mut self, _center: Vec2<i32>) {}
    fn process_and_transmit_msg(&mut self, _msg: ActionMsg) {}
}
















pub struct Layers {
    layers: Vec<Box<Element>>,

    frame: Frame<i32>,
}

impl Layers {
    pub fn new() -> Box<Self> {
        Box::new(Layers {
            layers: Vec::new(),
            frame: Frame::new()
        })
    }

    pub fn push(&mut self, element: Box<Element>) {
        self.layers.push(element);
    }

    pub fn insert(&mut self, index: usize, element: Box<Element>) {
        if index >= self.layers.len() {
            self.layers.push(element);
        } else {
            self.layers.insert(index, element);
        }
    }
}

impl Element for Layers {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        for el in &mut self.layers {
            el.setup(ui);
        }
    }

    fn stop(&self) {
        for el in &self.layers {
            el.stop();
        }
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {
        for n in 0..self.layers.len() {
            self.layers[n].build_window(ui);
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>) {
        self.frame = frame;
        for el in &mut self.layers {
            el.set_frame(frame);
        }
    }

    fn get_min_size(&self) -> Vec2<i32> {
        let mut res = Vec2::zero();
        for el in &self.layers {
            let min = el.get_min_size();
            if res.x < min.x { res.x = min.x }
            if res.y < min.y { res.y = min.y }
        }
        res
    }
    fn get_max_size(&self) -> Vec2<i32> {
        use std::i32;
        let mut res = Vec2{
            x: i32::MAX, y: i32::MAX
        };
        for el in &self.layers {
            let max = el.get_max_size();
            if res.x > max.x { res.x = max.x }
            if res.y > max.y { res.y = max.y }
        }
        res
    }

    fn set_window_center(&mut self, center: Vec2<i32>) {
        for el in &mut self.layers {
            el.set_window_center(center);
        }
    }

    fn process_and_transmit_msg(&mut self, msg: ActionMsg) {
        for layer in &mut self.layers {
            layer.process_and_transmit_msg(msg.clone());
        }
    }
}





















pub enum ListAlignment {
    Horizontal,
    Vertical,
}

pub enum ListElementSize {
    Weight(f64),
    Absolute(i32),
}


pub struct List {
    elements: Vec<Box<Element>>,
    ring: Ring<i32>,
    alignment: ListAlignment,

    frame: Frame<i32>,
    global_center: Vec2<i32>,
}

impl List {
    pub fn new(alignment: ListAlignment) -> Box<Self> {
        Box::new(List {
            elements: Vec::new(),
            ring: Ring::new(),
            alignment,
            frame: Frame::new(),
            global_center: Vec2::zero(),
        })
    }

    pub fn push(&mut self, element: Box<Element>) {
        //let n = self.elements.len();
        self.insert(0, element);
    }

    pub fn insert(&mut self, index: usize, mut element: Box<Element>) {

        element.set_window_center(self.global_center);

        if index >= self.elements.len() {
            self.elements.push(element);
        } else {
            self.elements.insert(index, element);
        }
        if DEBUG { println!("inserting into ring...");}
        self.ring.insert(index, RingElement::new());
        if DEBUG { println!("... inserting into ring done.");}

        self.rescale_elements();
    }

    fn rescale_elements(&mut self) {
        if DEBUG { println!("rescaling...");}

        match self.alignment {
            ListAlignment::Horizontal => {
                self.ring.resize(self.frame.width());
                for ix in 0..self.elements.len() {
                    let el = &mut self.elements[ix];
                    let x0 = self.ring.get_sum(ix);
                    let x1 = self.ring.get_sum(ix+1);
                    el.set_frame(Frame{
                        p0: Vec2{x: x0 + self.frame.p0.x, y: self.frame.p0.y},
                        p1: Vec2{x: x1 + self.frame.p0.x, y: self.frame.p1.y}
                    });
                }
            },
            ListAlignment::Vertical => {
                for ix in 0..self.elements.len() {
                    self.ring.resize(self.frame.height());
                    let el = &mut self.elements[ix];
                    let y0 = self.ring.get_sum(ix);
                    let y1 = self.ring.get_sum(ix+1);
                    el.set_frame(Frame{
                        p0: Vec2{x: self.frame.p0.x, y: y0 + self.frame.p0.y},
                        p1: Vec2{x: self.frame.p1.x, y: y1 + self.frame.p0.y}
                    });
                }
            },
        }
        if DEBUG { println!("... rescaling done.");}
    }
}

impl Element for List {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        for el in &mut self.elements {
            el.setup(ui);
        }
    }

    fn stop(&self) {
        for el in &self.elements {
            el.stop();
        }
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {
        for el in &self.elements {
            el.build_window(ui);
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>) {
        self.frame = frame;
        self.rescale_elements();
    }

    fn get_min_size(&self) -> Vec2<i32> {
        let mut min = Vec2::zero();
        for el in &self.elements {
            let tmp = el.get_min_size();
            match self.alignment {
                ListAlignment::Horizontal => {
                    min.x += tmp.x;
                    if min.y < tmp.y { min.y = tmp.y; }
                },
                ListAlignment::Vertical   => {
                    min.y += tmp.y;
                    if min.x < tmp.x { min.x = tmp.x; }
                },
            }
        }
        min
    }
    fn get_max_size(&self) -> Vec2<i32> {
        let mut max = Vec2::zero();
        for el in &self.elements {
            let tmp = el.get_max_size();
            match self.alignment {
                ListAlignment::Horizontal => {
                    max.x += tmp.x;
                    if max.y > tmp.y { max.y = tmp.y; }
                },
                ListAlignment::Vertical   => {
                    max.y += tmp.y;
                    if max.x < tmp.x { max.x = tmp.x; }
                },
            }
        }
        max
    }

    fn set_window_center(&mut self, center: Vec2<i32>) {
        self.global_center = center;
        let n = self.elements.len();
        for ix in 0..n {
            self.elements[ix].set_window_center(center);
        }
    }

    fn process_and_transmit_msg(&mut self, msg: ActionMsg) {
        for el in &mut self.elements {
            el.process_and_transmit_msg(msg.clone());
        }
    }
}

























pub enum PadElementSize {
    Absolute(i32, i32),
    Relative(f64, f64),
    AbsoluteNeg(i32, i32),
    RelativeNeg(f64, f64),
}

pub enum PadAlignment {
    Center,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

widget_ids!(
    struct PadIds {
        background,
    }
);


pub struct Pad {
    element: Box<Element>,
    pad_size: PadElementSize,
    alignment: PadAlignment,

    frame: Frame<i32>,
    global_center: Vec2<i32>,

    ids: Option<PadIds>,
    background: Background,
}


impl Pad {
    pub fn new(element: Box<Element>, alignment: PadAlignment, size: PadElementSize) -> Box<Self> {
        Box::new(Pad {
            element,
            alignment,
            pad_size: size,
            frame: Frame::new(),
            global_center: Vec2::zero(),
            ids: None,
            background: Background::None,
        })
    }
}

impl Backgroundable for Pad {
    fn with_background(mut self, bg: Background) -> Box<Self> {
        self.background = bg;
        Box::new(self)
    }
}

impl Element for Pad {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.ids = Some(PadIds::new(ui.widget_id_generator()));
        self.element.setup(ui);
    }

    fn stop(&self) {
        self.element.stop();
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {
        use conrod::{Widget, Positionable};

        if let Some(ref ids) = self.ids {

            let center = self.frame.center() - self.global_center;

            match self.background {
                Background::None => (),
                Background::Color(color) => {
                    let mut rect = conrod::widget::Rectangle::fill_with(
                        [self.frame.width() as f64, self.frame.height() as f64],
                        color
                    ).x_y(center.x as f64, center.y as f64);
                    rect.set(ids.background, ui);
                }
            }
            self.element.build_window(ui);
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }

    #[allow(unreachable_patterns)]
    fn set_frame(&mut self, frame: Frame<i32>) {
        self.frame = frame;
        use self::PadAlignment::*;

        let min = self.element.get_min_size();

        // map relative values to absolute pixel
        let s = {
            let tmp = self.frame.size();
            Vec2{ x: tmp.x as f64, y: tmp.y as f64}
        };

        let mut v = match self.pad_size {
            PadElementSize::Absolute(x,y) => Vec2{x,y},
            PadElementSize::Relative(x,y) => {
                let v = Vec2{x,y};
                let v2 =v.el_mul(s);
                Vec2{ x: v2.x as i32, y: v2.y as i32 }
            },
            PadElementSize::AbsoluteNeg(x, y) => {
                let s = self.frame.size();
                Vec2{x: s.x - x, y: s.y - y}
            },
            PadElementSize::RelativeNeg(x, y) => {
                let v = Vec2{x,y};
                let v2 = s - v.el_mul(s);
                Vec2{x: v2.x as i32, y: v2.y as i32}
            }
        };
        if v.x < min.x { v.x = min.x; }
        if v.y < min.y { v.y = min.y; }

        let center = self.frame.center();

        let frame: Frame<i32> = match self.alignment {
            Center => {
                Frame{
                    p0: center - v/2,
                    p1: center + v/2,
                }
            },
            BottomLeft => {
                Frame {
                    p0: self.frame.p0,
                    p1: self.frame.p0 + v,
                }
            },
            Bottom => {
                let midx = (self.frame.p1.x + self.frame.p0.x)/2;
                Frame {
                    p0: Vec2{x: midx - v.x/2, y: self.frame.p0.y},
                    p1: Vec2{x: midx + v.x/2, y: self.frame.p0.y + v.y}
                }
            },
            BottomRight => {
                Frame {
                    p0: Vec2{x: self.frame.p1.x - v.x, y: self.frame.p0.y},
                    p1: Vec2{x: self.frame.p1.x, y: self.frame.p0.y + v.y}
                }
            },
            Right => {
                let midy = (self.frame.p1.y + self.frame.p0.y)/2;
                Frame {
                    p0: Vec2{x: self.frame.p1.x - v.x, y: midy - v.y/2},
                    p1: Vec2{x: self.frame.p1.x, y: midy + v.y/2}
                }
            },
            TopRight => {
                Frame{
                    p0: self.frame.p1 - v,
                    p1: self.frame.p1,
                }
            },
            Top => {
                let midx = (self.frame.p1.x + self.frame.p0.x)/2;
                Frame {
                    p0: Vec2{x: midx - v.x/2, y: self.frame.p1.y - v.y},
                    p1: Vec2{x: midx + v.x/2, y: self.frame.p1.y}
                }
            },
            TopLeft => {
                Frame{
                    p0: Vec2{x: self.frame.p0.x, y: self.frame.p1.y - v.y},
                    p1: Vec2{x: self.frame.p0.x + v.x, y: self.frame.p1.y},
                }
            },
            Left => {
                let midy = (self.frame.p1.y + self.frame.p0.y)/2;
                Frame {
                    p0: Vec2{x: self.frame.p0.x, y: midy - v.y/2},
                    p1: Vec2{x: self.frame.p0.x + v.x, y: midy + v.y/2}
                }
            },
            _ => self.frame
        };

        self.element.set_frame(frame);
    }

    fn get_min_size(&self) -> Vec2<i32> {
        // TODO not yet correct. Need to consider relative size as well
        // TODO ... maybe not?
        self.element.get_min_size()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        // TODO not yet correct. Need to consider relative size as well
        // TODO ... maybe not?
        self.element.get_max_size()
    }

    fn set_window_center(&mut self, center: Vec2<i32>) {
        self.global_center = center;
        self.element.set_window_center(center);
    }

    fn process_and_transmit_msg(&mut self, msg: ActionMsg) {
        self.element.process_and_transmit_msg(msg);
    }
}



use elements::{*, action::*};










const DEBUG: bool = false;




















/*
d88888b .88b  d88. d8888b. d888888b db    db
88'     88'YbdP`88 88  `8D `~~88~~' `8b  d8'
88ooooo 88  88  88 88oodD'    88     `8bd8'
88~~~~~ 88  88  88 88~~~      88       88
88.     88  88  88 88         88       88
Y88888P YP  YP  YP 88         YP       YP


*/





pub struct Empty {
    is_setup: bool,
    frame: Frame<i32>,
    window_center: Vec2<i32>
}
impl Empty {
    pub fn new() -> Box<Self> {
        Box::new(Empty{
            is_setup: false,
            frame: Frame::new(),
            window_center: Vec2::zero()
        })
    }
}
impl Element for Empty {
    fn setup(&mut self, _ui: &mut conrod::Ui) { self.is_setup = true }
    fn is_setup(&self) -> bool { self.is_setup }

    fn build_window(&self, _ui: &mut conrod::UiCell, _ressources: &WindowRessources) {}

    fn get_frame(&self) -> Frame<i32> { self.frame }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.frame = frame;
        self.window_center = window_center;

    }

    fn transmit_msg(&mut self, _msg: ActionMsg, _stop: bool) {}
}













/*
db       .d8b.  db    db d88888b d8888b. .d8888.
88      d8' `8b `8b  d8' 88'     88  `8D 88'  YP
88      88ooo88  `8bd8'  88ooooo 88oobY' `8bo.
88      88~~~88    88    88~~~~~ 88`8b     `Y8b.
88booo. 88   88    88    88.     88 `88. db   8D
Y88888P YP   YP    YP    Y88888P 88   YD `8888Y'


*/



pub struct Layers {
    layers: Vec<Box<Element>>,

    is_setup: bool,
    frame: Frame<i32>,
}

impl Layers {
    pub fn new() -> Box<Self> {
        Box::new(Layers {
            layers: Vec::new(),
            is_setup: false,
            frame: Frame::new()
        })
    }

    pub fn push(&mut self, element: Box<Element>) {
        self.layers.push(element);
        self.is_setup = false;
    }

    pub fn insert(&mut self, index: usize, element: Box<Element>) {
        if index >= self.layers.len() {
            self.layers.push(element);
        } else {
            self.layers.insert(index, element);
        }
        self.is_setup = false;
    }

    pub fn pop(&mut self) -> Option<Box<Element>> {
        self.layers.pop()
    }

    pub fn remove(&mut self, index: usize) -> Box<Element> {
        self.layers.remove(index)
    }
}

impl Element for Layers {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        for el in &mut self.layers {
            if !el.is_setup() {
                el.setup(ui);
            }
        }
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool {
        let mut setup = self.is_setup;
        for el in &self.layers {
            if !el.is_setup() { setup = false; }
        }
        if DEBUG { println!("is layers setup? {}",setup); }
        setup
    }

    fn stop(&mut self) {
        for el in &mut self.layers {
            el.stop();
        }
    }
    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        for n in 0..self.layers.len() {
            self.layers[n].build_window(ui, ressources);
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.frame = frame;
        for el in &mut self.layers {
            el.set_frame(frame, window_center);
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

    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) {
        if !stop {
            for layer in &mut self.layers {
                layer.transmit_msg(msg.clone(), false);
            }
        }
    }
}

















/*
db      d888888b .d8888. d888888b
88        `88'   88'  YP `~~88~~'
88         88    `8bo.      88
88         88      `Y8b.    88
88booo.   .88.   db   8D    88
Y88888P Y888888P `8888Y'    YP


*/



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

    is_setup: bool,
    frame: Frame<i32>,
    global_center: Vec2<i32>,
}

impl List {
    pub fn new(alignment: ListAlignment) -> Box<Self> {
        Box::new(List {
            elements: Vec::new(),
            ring: Ring::new(),
            alignment,
            is_setup: false,
            frame: Frame::new(),
            global_center: Vec2::zero(),
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
        if DEBUG { println!("inserting into ring...");}
        self.ring.insert(index, RingElement::new());
        if DEBUG { println!("... inserting into ring done.");}

        self.rescale_elements();
        self.is_setup = false;
    }

    pub fn pop(&mut self) -> Option<Box<Element>> {
        let el = self.elements.pop();
        let _ = self.ring.pop();
        self.rescale_elements();
        el
    }

    pub fn remove(&mut self, index: usize) -> Box<Element> {
        let el = self.elements.remove(index);
        let _ = self.ring.remove(index);
        self.rescale_elements();
        el
    }

    fn rescale_elements(&mut self) {
        if DEBUG { println!("rescaling...");}

        let n = self.elements.len();

        match self.alignment {
            ListAlignment::Horizontal => {
                self.ring.resize(self.frame.width());
                for ix in 0..n {
                    let el = &mut self.elements[ix];
                    let x0 = self.ring.get_sum(ix);
                    let x1 = self.ring.get_sum(ix+1);
                    el.set_frame(Frame{
                        p0: Vec2{x: x0 + self.frame.p0.x, y: self.frame.p0.y},
                        p1: Vec2{x: x1 + self.frame.p0.x, y: self.frame.p1.y}
                    }, self.global_center);
                }
            },
            ListAlignment::Vertical => {
                for ix in 0..n {
                    self.ring.resize(self.frame.height());
                    let el = &mut self.elements[n-1-ix];
                    let y0 = self.ring.get_sum(ix);
                    let y1 = self.ring.get_sum(ix+1);
                    el.set_frame(Frame{
                        p0: Vec2{x: self.frame.p0.x, y: y0 + self.frame.p0.y},
                        p1: Vec2{x: self.frame.p1.x, y: y1 + self.frame.p0.y}
                    }, self.global_center);
                }
            },
        }
        if DEBUG { println!("... rescaling done.");}
    }
}

impl Element for List {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        for el in &mut self.elements {
            if !el.is_setup() { el.setup(ui); }
        }
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool {
        let mut setup = self.is_setup;
        for el in &self.elements {
            if !el.is_setup() { setup = false; }
        }
        if DEBUG { println!("List is setup? {}", setup); }
        setup
    }

    fn stop(&mut self) {
        for el in &mut self.elements {
            el.stop();
        }
    }
    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        for el in &self.elements {
            el.build_window(ui, ressources);
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
                    max.x = if tmp.x == i32::MAX {
                        i32::MAX
                    } else {
                        max.x + tmp.x
                    };
                    if max.y > tmp.y { max.y = tmp.y; }
                },
                ListAlignment::Vertical   => {
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

    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) {
        if !stop {
            for el in &mut self.elements {
                el.transmit_msg(msg.clone(), false);
            }
        }
    }
}






















/*
d8888b.  .d8b.  d8888b.
88  `8D d8' `8b 88  `8D
88oodD' 88ooo88 88   88
88~~~   88~~~88 88   88
88      88   88 88  .8D
88      YP   YP Y8888D'


*/


#[derive(Debug, Copy, Clone)]
pub enum PadElementSize {
    Positive(Dim, Dim),
    Negative(Dim, Dim),
}

#[derive(Debug, Copy, Clone)]
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
    XY(Dim, Dim),
}

pub struct Pad {
    element: Box<Element>,
    pad_size: PadElementSize,
    alignment: PadAlignment,

    is_setup: bool,
    frame: Frame<i32>,
    global_center: Vec2<i32>,

    original_pad_size: PadElementSize,
    original_alignment: PadAlignment,
}


impl Pad {
    pub fn new(element: Box<Element>, alignment: PadAlignment, size: PadElementSize) -> Box<Self> {
        //println!("Pad with size {:?}", size);
        Box::new(Pad {
            element,
            alignment,
            pad_size: size,
            is_setup: false,
            frame: Frame::new(),
            global_center: Vec2::zero(),
            original_pad_size: size,
            original_alignment: alignment,
        })
    }


    fn update_original_pad_size(&mut self) {
        self.original_pad_size = self.pad_size;
        self.original_alignment = self.alignment;
    }

    fn rescale(&mut self, frame: Frame<i32>, window_center: Vec2<i32>, limit: bool) {
        self.global_center = window_center;
        self.frame = frame;
        use self::PadAlignment::*;

        let min = self.element.get_min_size();

        // map relative values to absolute pixel
        let s = self.frame.size();

        let mut v = match self.pad_size {
            PadElementSize::Positive(ref x, ref y) => {
                let xx = match x {
                    &Dim::Absolute(ix) => ix,
                    &Dim::Relative(fx) => (s.x as f64 * fx) as i32
                };
                let yy = match y {
                    &Dim::Absolute(iy) => iy,
                    &Dim::Relative(fy) => (s.y as f64 * fy) as i32
                };
                Vec2{x: xx, y: yy}
            },
            PadElementSize::Negative(ref x, ref y) => {
                let xx = match x {
                    &Dim::Absolute(ix) => s.x - ix,
                    &Dim::Relative(fx) => (s.x as f64 * (1.0 - fx)) as i32
                };
                let yy = match y {
                    &Dim::Absolute(iy) => s.y - iy,
                    &Dim::Relative(fy) => (s.y as f64 * (1.0 - fy)) as i32
                };
                Vec2{x: xx, y: yy}
            }
        };

        if limit {
            if v.x < min.x { v.x = min.x; }
            if v.y < min.y { v.y = min.y; }
        }

        let center = self.frame.center();
        let w = self.frame.width();
        let h = self.frame.height();

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
            XY(ax,ay) => {
                let xx = match ax {
                    Dim::Absolute(ix) => ix,
                    Dim::Relative(fx) => (fx * w as f64/2.0) as i32,
                };
                let yy = match ay {
                    Dim::Absolute(iy) => iy,
                    Dim::Relative(fy) => (fy * h as f64/2.0) as i32,
                };
                let vv = Vec2{x: xx, y: yy};
                Frame {
                    p0: center - v/2 + vv,
                    p1: center + v/2 + vv,
                }
            }
        };

        self.element.set_frame(frame, window_center);
    }
}


// ! ////////////////////////////////////////////////////////////////////////////////////////
// ! need to change pad_size, not frame!!! //////////////////////////////////////////////////
// ! ////////////////////////////////////////////////////////////////////////////////////////
impl Animateable for Pad {
    fn animate_size(&mut self, xy: (Dim,Dim)) {

        let frame = self.frame;
        let w = frame.width();
        let h = frame.height();

        //println!("w {}, h {}",w,h);
        //println!("self.pad_size {:?}", self.pad_size);

        let (px,py) = match self.original_pad_size {
            PadElementSize::Positive(dx,dy) => {
                let xx = match dx {
                    Dim::Absolute(ix) => ix as f64,
                    Dim::Relative(fx) => fx * w as f64
                };
                let yy = match dy {
                    Dim::Absolute(iy) => iy as f64,
                    Dim::Relative(iy) => iy * h as f64
                };
                (xx,yy)
            },
            PadElementSize::Negative(dx,dy) => {
                let xx = match dx {
                    Dim::Absolute(ix) => w as f64 - ix as f64,
                    Dim::Relative(fx) => w as f64 - fx * w as f64
                };
                let yy = match dy {
                    Dim::Absolute(iy) => h as f64 - iy as f64,
                    Dim::Relative(fy) => h as f64 - fy * h as f64
                };
                (xx,yy)
            }
        };

        let (x,y) = {
            let xx = match xy.0 {
                Dim::Absolute(ix) => ix as f64,
                Dim::Relative(fx) => fx * px
            };
            let yy = match xy.1 {
                Dim::Absolute(iy) => iy as f64,
                Dim::Relative(fy) => fy * py
            };
            (xx,yy)
        };

        self.pad_size = PadElementSize::Positive(
            Dim::Absolute( (px+x) as i32),
            Dim::Absolute( (py+y) as i32) 
        );

        let center = self.global_center;
        self.rescale(frame, center, false);
    }

    fn animate_position(&mut self, xy: (Dim, Dim)) {
        let (x,y) = xy;

        self.alignment = PadAlignment::XY(x,y);
        
        let frame = self.frame;
        let center = self.global_center;
        self.rescale(frame, center, false);
    }

    fn reset(&mut self) {
        self.alignment = self.original_alignment;
        self.pad_size = self.original_pad_size;
        let center = self.global_center;
        let frame = self.frame;
        self.rescale(frame, center, true);
    }
}


impl Element for Pad {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        if !self.element.is_setup() { self.element.setup(ui); }
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool {
        self.is_setup && self.element.is_setup()
    }

    fn stop(&mut self) {
        self.element.stop();
    }
    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        self.element.build_window(ui, ressources);
        if DEBUG { println!("Pad build.");}
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }

    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.rescale(frame, window_center, true);
        self.update_original_pad_size();
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

    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) {
        if !stop { self.element.transmit_msg(msg, false); }
    }
}




use conrod;

use elements::*;





pub enum ListAlignment {
    Horizontal,
    Vertical,
}


pub struct List {
    elements: Vec<Box<Element>>,
    rel_sep: Vec<f64>,
    alignment: ListAlignment,
    frame: Frame<i32>,

    global_center: Vec2<i32>,
}

impl List {
    pub fn new(alignment: ListAlignment) -> Self {
        List {
            elements: Vec::new(),
            rel_sep: vec![0.0],
            alignment,
            frame: Frame::new(100,100),
            global_center: Vec2::zero(),
        }
    }

    pub fn add_element(&mut self, element: Box<Element>) {
        let n = self.elements.len();
        self.add_element_at(element, n);
    }

    pub fn add_element_at(&mut self, mut element: Box<Element>, index: usize) {

        element.set_window_center(self.global_center);

        let n = self.elements.len();
        let rel_sep = 1.0 / ((n+1) as f64);

        for ix in 0..n {
            self.rel_sep[ix+1] *=  n as f64 * rel_sep;
        }
        let last_sep = self.rel_sep[n];

        if index >= n {
            self.rel_sep.push(last_sep + rel_sep);
        } else {
            self.rel_sep.insert(index, last_sep + rel_sep);
        }

        match self.alignment {
            ListAlignment::Horizontal => {
                if index >= n {
                    self.elements.push(element);
                } else {
                    self.elements.insert(index, element);
                }

                for ix in 0..n {
                    let el = &mut self.elements[ix];
                    el.set_frame(Frame{
                        p0: Vec2{x: (self.rel_sep[ix]   * self.frame.width() as f64) as i32 + self.frame.p0.x, y: self.frame.p0.y},
                        p1: Vec2{x: (self.rel_sep[ix+1] * self.frame.width() as f64) as i32 + self.frame.p0.x, y: self.frame.p1.y}
                    });
                }
            },
            ListAlignment::Vertical => {
                if index >= n {
                    self.elements.push(element);
                } else {
                    self.elements.insert(index, element);
                }

                for ix in 0..n {
                    let el = &mut self.elements[ix];
                    el.set_frame(Frame{
                        p0: Vec2{x: self.frame.p0.x, y: (self.rel_sep[ix]   * self.frame.height() as f64) as i32 + self.frame.p0.y},
                        p1: Vec2{x: self.frame.p1.x, y: (self.rel_sep[ix+1] * self.frame.height() as f64) as i32 + self.frame.p0.y}
                    });
                }
            },
        }
    }
}

impl Element for List {
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

        let n = self.elements.len();

        match self.alignment {
            ListAlignment::Horizontal => {
                for ix in 0..n {
                    let el = &mut self.elements[ix];
                    el.set_frame(Frame{
                        p0: Vec2{x: (self.rel_sep[ix]   * self.frame.width() as f64) as i32 + self.frame.p0.x, y: self.frame.p0.y},
                        p1: Vec2{x: (self.rel_sep[ix+1] * self.frame.width() as f64) as i32 + self.frame.p0.x, y: self.frame.p1.y}
                    });
                }
            },
            ListAlignment::Vertical => {
                for ix in 0..n {
                    let el = &mut self.elements[ix];
                    el.set_frame(Frame{
                        p0: Vec2{x: self.frame.p0.x, y: (self.rel_sep[ix]   * self.frame.height() as f64) as i32 + self.frame.p0.y},
                        p1: Vec2{x: self.frame.p1.x, y: (self.rel_sep[ix+1] * self.frame.height() as f64) as i32 + self.frame.p0.y}
                    });
                }
            },
        }
    }

    fn set_window_center(&mut self, center: Vec2<i32>) {
        self.global_center = center;
        let n = self.elements.len();
        for ix in 0..n {
            self.elements[ix].set_window_center(center);
        }
    }
}









pub enum PadAlignment {
    CenterRel(Vec2<f64>),
    TopLeftRel(Vec2<f64>),
    TopRel(Vec2<f64>),
    TopRightRel(Vec2<f64>),
    RightRel(Vec2<f64>),
    BottomRightRel(Vec2<f64>),
    BottomRel(Vec2<f64>),
    BottomLeftRel(Vec2<f64>),
    LeftRel(Vec2<f64>),

    CenterAbs(Vec2<i32>),
    TopLeftAbs(Vec2<i32>),
    TopAbs(Vec2<i32>),
    TopRightAbs(Vec2<i32>),
    RightAbs(Vec2<i32>),
    BottomRightAbs(Vec2<i32>),
    BottomAbs(Vec2<i32>),
    BottomLeftAbs(Vec2<i32>),
    LeftAbs(Vec2<i32>),
}


pub struct Pad {
    element: Box<Element>,
    alignment: PadAlignment,

    frame: Frame<i32>,
    global_center: Vec2<i32>,
}


impl Pad {
    pub fn new(element: Box<Element>, alignment: PadAlignment) -> Self {
        Pad {
            element, alignment, frame: Frame::new(100,100), global_center: Vec2::zero()
        }
    }
}




fn map_f64_to_i32(v: Vec2<f64>) -> Vec2<i32> {
    Vec2{
        x: v.x as i32,
        y: v.y as i32,
    }
}

impl Element for Pad {
    fn stop(&self) {
        self.element.stop();
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {
        self.element.build_window(ui);
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>) {
        self.frame = frame;
        use self::PadAlignment::*;

        println!("{:?}", self.frame);

        // map relative values to absolute pixel
        let s = {
            let tmp = self.frame.size();
            Vec2{ x: tmp.x as f64, y: tmp.y as f64}
        };
        let absv = match self.alignment {
            CenterAbs(v) => CenterAbs(v),
            TopLeftAbs(v) => TopLeftAbs(v),
            TopAbs(v) => TopAbs(v),
            TopRightAbs(v) => TopRightAbs(v),
            RightAbs(v) => RightAbs(v),
            BottomRightAbs(v) => BottomRightAbs(v),
            BottomAbs(v) => BottomAbs(v),
            BottomLeftAbs(v) => BottomLeftAbs(v),
            LeftAbs(v) => LeftAbs(v),

            CenterRel(v) => CenterAbs(map_f64_to_i32(v.el_mul(s))),
            TopLeftRel(v) => TopLeftAbs(map_f64_to_i32(v.el_mul(s))),
            TopRel(v) => TopAbs(map_f64_to_i32(v.el_mul(s))),
            TopRightRel(v) => TopRightAbs(map_f64_to_i32(v.el_mul(s))),
            RightRel(v) => RightAbs(map_f64_to_i32(v.el_mul(s))),
            BottomRightRel(v) => BottomRightAbs(map_f64_to_i32(v.el_mul(s))),
            BottomRel(v) => BottomAbs(map_f64_to_i32(v.el_mul(s))),
            BottomLeftRel(v) => BottomLeftAbs(map_f64_to_i32(v.el_mul(s))),
            LeftRel(v) => LeftAbs(map_f64_to_i32(v.el_mul(s))),
        };



        let frame: Frame<i32> = match absv {
            CenterAbs(v) => {
                let center = self.frame.center();
                Frame{
                    p0: center - v/2,
                    p1: center + v/2,
                }
            },
            BottomLeftAbs(v) => {
                Frame {
                    p0: self.frame.p0,
                    p1: self.frame.p0 + v,
                }
            },
            BottomAbs(v) => {
                let midx = (self.frame.p1.x + self.frame.p0.x)/2;
                Frame {
                    p0: Vec2{x: midx - v.x/2, y: self.frame.p0.y},
                    p1: Vec2{x: midx + v.x/2, y: self.frame.p0.y + v.y}
                }
            },
            BottomRightAbs(v) => {
                Frame {
                    p0: Vec2{x: self.frame.p1.x - v.x, y: self.frame.p0.y},
                    p1: Vec2{x: self.frame.p1.x, y: self.frame.p0.y + v.y}
                }
            },
            RightAbs(v) => {
                let midy = (self.frame.p1.y + self.frame.p0.y)/2;
                Frame {
                    p0: Vec2{x: self.frame.p1.x - v.x, y: midy - v.y/2},
                    p1: Vec2{x: self.frame.p1.x, y: midy + v.y/2}
                }
            },
            TopRightAbs(v) => {
                Frame{
                    p0: self.frame.p1 - v,
                    p1: self.frame.p1,
                }
            },
            TopAbs(v) => {
                let midx = (self.frame.p1.x + self.frame.p0.x)/2;
                Frame {
                    p0: Vec2{x: midx - v.x/2, y: self.frame.p1.y - v.y},
                    p1: Vec2{x: midx + v.x/2, y: self.frame.p1.y}
                }
            },
            TopLeftAbs(v) => {
                Frame{
                    p0: Vec2{x: self.frame.p0.x, y: self.frame.p1.y - v.y},
                    p1: Vec2{x: self.frame.p0.x + v.x, y: self.frame.p1.y},
                }
            },
            LeftAbs(v) => {
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

    fn set_window_center(&mut self, center: Vec2<i32>) {
        self.global_center = center;
        self.element.set_window_center(center);
    }
}
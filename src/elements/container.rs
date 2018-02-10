


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
    pub fn new(alignment: ListAlignment) -> Box<Self> {
        Box::new(List {
            elements: Vec::new(),
            rel_sep: vec![0.0],
            alignment,
            frame: Frame::new(100,100),
            global_center: Vec2::zero(),
        })
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
            frame: Frame::new(100,100),
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




fn map_f64_to_i32(v: Vec2<f64>) -> Vec2<i32> {
    Vec2{
        x: v.x as i32,
        y: v.y as i32,
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
                map_f64_to_i32(v.el_mul(s))
            },
            PadElementSize::AbsoluteNeg(x, y) => {
                let s = self.frame.size();
                Vec2{x: s.x - x, y: s.y - y}
            },
            PadElementSize::RelativeNeg(x, y) => {
                let v = Vec2{x,y};
                map_f64_to_i32(s - v.el_mul(s))
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
        self.element.get_min_size()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        // TODO not yet correct. Need to consider relative size as well
        self.element.get_max_size()
    }

    fn set_window_center(&mut self, center: Vec2<i32>) {
        self.global_center = center;
        self.element.set_window_center(center);
    }
}
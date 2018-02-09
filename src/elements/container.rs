


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
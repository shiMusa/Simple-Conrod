


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
        let N = self.elements.len();
        self.add_element_at(element, N);
    }

    pub fn add_element_at(&mut self, mut element: Box<Element>, index: usize) {

        element.set_window_center(self.global_center);

        let N = self.elements.len();
        let rel_sep = 1.0 / ((N+1) as f64);

        let mut sep_sum = 0f64;
        for ix in 0..N {
            self.rel_sep[ix+1] *= rel_sep;
            sep_sum += self.rel_sep[ix+1];
        }
        if index >= N {
            self.rel_sep.push(sep_sum + rel_sep);
        } else {
            self.rel_sep.insert(index, sep_sum + rel_sep);
        }

        match self.alignment {
            ListAlignment::Horizontal => {
                if index >= N {
                    self.elements.push(element);
                } else {
                    self.elements.insert(index, element);
                }

                let mut sum = 0f64;
                for ix in 0..N {
                    let el = &mut self.elements[ix];
                    el.set_frame(Frame{
                        p0: Vec2{x: ((self.rel_sep[ix]   + sum) * self.frame.width() as f64) as i32 + self.frame.p0.x, y: self.frame.p0.y},
                        p1: Vec2{x: ((self.rel_sep[ix+1] + sum) * self.frame.width() as f64) as i32 + self.frame.p0.x, y: self.frame.p1.y}
                    });
                    sum += self.rel_sep[ix];
                }
            },
            ListAlignment::Vertical => {
                if index >= N {
                    self.elements.push(element);
                } else {
                    self.elements.insert(index, element);
                }

                let mut sum = 0f64;
                for ix in 0..N {
                    let el = &mut self.elements[ix];
                    el.set_frame(Frame{
                        p0: Vec2{x: self.frame.p0.x, y: ((self.rel_sep[ix]   + sum) * self.frame.height() as f64) as i32 + self.frame.p0.y},
                        p1: Vec2{x: self.frame.p1.x, y: ((self.rel_sep[ix+1] + sum) * self.frame.height() as f64) as i32 + self.frame.p0.y}
                    });
                    sum += self.rel_sep[ix];
                }
            },
        }

        println!("{:?}", self.rel_sep);
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

        let N = self.elements.len();

        match self.alignment {
            ListAlignment::Horizontal => {
                let mut sum = 0f64;
                for ix in 0..N {
                    let el = &mut self.elements[ix];
                    el.set_frame(Frame{
                        p0: Vec2{x: ((self.rel_sep[ix]   + sum) * self.frame.width() as f64) as i32 + self.frame.p0.x, y: self.frame.p0.y},
                        p1: Vec2{x: ((self.rel_sep[ix+1] + sum) * self.frame.width() as f64) as i32 + self.frame.p0.x, y: self.frame.p1.y}
                    });
                    sum += self.rel_sep[ix];
                }
            },
            ListAlignment::Vertical => {
                let mut sum = 0f64;
                for ix in 0..N {
                    let el = &mut self.elements[ix];
                    el.set_frame(Frame{
                        p0: Vec2{x: self.frame.p0.x, y: ((self.rel_sep[ix]   + sum) * self.frame.height() as f64) as i32 + self.frame.p0.y},
                        p1: Vec2{x: self.frame.p1.x, y: ((self.rel_sep[ix+1] + sum) * self.frame.height() as f64) as i32 + self.frame.p0.y}
                    });
                    sum += self.rel_sep[ix];
                }
            },
        }
    }

    fn set_window_center(&mut self, center: Vec2<i32>) {
        self.global_center = center;
        let N = self.elements.len();
        for ix in 0..N {
            self.elements[ix].set_window_center(center);
        }
    }
}
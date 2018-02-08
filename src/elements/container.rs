


use conrod;

use elements::*;





pub enum Alignment {
    Center,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    Relative(f64,f64),
}




struct ListElement {
    element: Box<Element>,
    align: Alignment,
    size: Vec2f64,
    rel_pos: Vec2f64,
}

impl ListElement {
    fn resize(&mut self, size: Vec2u32) {
        let pix_size: Vec2u32 = (
            (self.size.0 * size.0 as f64) as u32,
            (self.size.1 * size.1 as f64) as u32
        );
        println!("ListElement: resize to pixel_size {:?}", pix_size);
        self.element.resize(pix_size);
    }
    fn repos(&mut self, rel_pos: Vec2f64) {
        println!("Listelement: repos to position {:?}", rel_pos);
        self.rel_pos = rel_pos;
    }
}


pub enum ListAlignment {
    Horizontal,
    Vertical,
}


pub struct List {
    elements: Vec<ListElement>,
    pixel_size: Vec2u32,
    pos: Vec2f64,
    rel_separations: Vec<f64>,
    alignment: ListAlignment,
}

impl List {
    pub fn new(alignment: ListAlignment) -> Self {
        List {
            elements: Vec::new(),
            pixel_size:(100, 100),
            pos: (0.0,0.0),
            rel_separations: Vec::new(),
            alignment
        }
    }

    pub fn add_element(&mut self, element: Box<Element>) {
        let N = self.elements.len();
        self.add_element_at(element, N);
    }

    pub fn add_element_at(&mut self, element: Box<Element>, index: usize) {

        let N = self.elements.len();
        let rel_sep = 1.0 / ((N+1) as f64);
        let rescale = (N as f64)/((N+1) as f64);

        match self.alignment {
            ListAlignment::Horizontal => {
                for mut el in &mut self.elements {
                    el.size.0 *= rescale;
                    el.resize(self.pixel_size);
                }

                if index >= N {
                    self.elements.push(ListElement{
                        element, align: Alignment::Center, size: (rel_sep, 1f64)
                    });
                } else {
                    self.elements.insert(index, ListElement{
                        element, align: Alignment::Center, size: (rel_sep, 1f64)
                    });
                }
            },
            ListAlignment::Vertical => {
                for mut el in &mut self.elements {
                    el.size.1 *= rescale;
                    el.resize(self.pixel_size);
                }

                if index >= N {
                    self.elements.push(ListElement{
                        element, align: Alignment::Center, size: (1f64, rel_sep)
                    });
                } else {
                    self.elements.insert(index, ListElement{
                        element, align: Alignment::Center, size: (1f64, rel_sep)
                    });
                }
            },
        }
    }
}

impl Element for List {
    fn stop(&self) {
        for el in &self.elements {
            el.element.stop();
        }
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {
        for el in &self.elements {
            el.element.build_window(ui);
        }
    }

    fn get_size(&self) -> Vec2u32 {
        self.pixel_size
    }
    fn resize(&mut self, size: Vec2u32) {
        self.pixel_size = size;
        for el in &mut self.elements {
            el.resize(size);
        }
    }

    fn get_position(&self) -> Vec2f64 {
        self.pos
    }
    fn reposition(&mut self, pos: Vec2f64) {
        self.pos = pos;
    }
}
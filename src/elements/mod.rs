

pub mod container;
pub mod basic;
pub mod action;


use conrod;
use conrod::backend::glium::glium::{self, Surface};
use conrod::position::{Rect, Range};
use time;
use num::{Num, NumCast};
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Debug, Formatter, Result};
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::HashMap;
use std::path::Path;

use find_folder;
use image;



const DEBUG: bool = false;




use elements::action::*;
















/*
d8888b. d888888b d8b   db  d888b  d88888b db      d88888b .88b  d88. d88888b d8b   db d888888b
88  `8D   `88'   888o  88 88' Y8b 88'     88      88'     88'YbdP`88 88'     888o  88 `~~88~~'
88oobY'    88    88V8o 88 88      88ooooo 88      88ooooo 88  88  88 88ooooo 88V8o 88    88
88`8b      88    88 V8o88 88  ooo 88~~~~~ 88      88~~~~~ 88  88  88 88~~~~~ 88 V8o88    88
88 `88.   .88.   88  V888 88. ~8~ 88.     88booo. 88.     88  88  88 88.     88  V888    88
88   YD Y888888P VP   V8P  Y888P  Y88888P Y88888P Y88888P YP  YP  YP Y88888P VP   V8P    YP


*/




#[derive(Debug, Copy, Clone)]
pub enum RingElementSize<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    Absolute(T),
    Relative(f64),
}
impl<T> RingElementSize<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    pub fn to_value(& self, cmp_to: T) -> T {
        match self {
            &RingElementSize::Absolute(x) => x,
            &RingElementSize::Relative(x) => T::from(cmp_to.to_f64().unwrap() * x).unwrap(),
        }
    }
}



#[derive(Debug, Copy, Clone)]
pub struct RingElement<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    min: RingElementSize<T>,
    max: RingElementSize<T>,
    size: RingElementSize<T>
}
impl<T> RingElement<T> where T: Num + NumCast + PartialOrd + Copy + Debug{
    pub fn new_with_min_max(
        min: RingElementSize<T>,
        max: RingElementSize<T>) -> Self {
        RingElement {
            min, max, size: RingElementSize::Relative(0.5)
        }
    }

    pub fn new() -> Self {
        RingElement::new_with_min_max(
            RingElementSize::Relative(0.0),
            RingElementSize::Relative(1.0),
        )
    }

    pub fn get_min(&self, cmp: T) -> T {
        let min = self.min.to_value(cmp);
        let max = self.max.to_value(cmp);

        if max < min { return max; }
        min
    }

    pub fn get_max(&self, cmp: T) -> T {
        let min = self.min.to_value(cmp);
        let max = self.max.to_value(cmp);

        if max < min { return min; }
        max
    }

    pub fn get_size(&self, cmp: T) -> T {
        self.size.to_value(cmp)
    }

    pub fn is_at_max(&self, cmp: T) -> bool {
        let size = self.size.to_value(cmp);
        let max = self.get_max(cmp);

        if DEBUG {
            println!("is at max? size {:?}, max {:?}", size, max);
        }

        if size >= max {
            if DEBUG { println!("   yes.");}
            true
        } else {
            if DEBUG { println!("   no.");}
            false
        }
    }

    pub fn shrink_to_min(&mut self, cmp: T) {
        if DEBUG {
            println!("shrinking size {:?} to min {:?}", self.size, self.min);
        }

        self.size = if self.get_min(cmp) > self.get_max(cmp) {
            if DEBUG { println!("min > max, so set to max");}
            self.max
        } else {
            self.min
        };
        if DEBUG { println!("shrinked? size {:?} to min {:?}", self.size, self.min);}
    }

    pub fn grow(&mut self, cmp: T, grow_by: T) -> T {
        if DEBUG { println!("growing size {:?}, max {:?} by {:?}", self.size.to_value(cmp), self.get_max(cmp), grow_by);}
        if self.is_at_max(cmp) {
            if DEBUG { println!("cannot grow. already at max.");}
            return grow_by
        }
        let s = self.size.to_value(cmp);
        self.size = RingElementSize::Absolute(s + grow_by);

        if self.is_at_max(cmp) {
            let rem = self.size.to_value(cmp) - self.get_max(cmp);
            self.size = self.max;
            if DEBUG { println!("grown? size {:?}, max {:?}, rem {:?}", self.size.to_value(cmp), self.get_max(cmp), rem);}
            return rem;
        }
        if DEBUG { println!("everything absorbed.");}
        T::from(0).unwrap()
    }
}



/*
d8888b. d888888b d8b   db  d888b
88  `8D   `88'   888o  88 88' Y8b
88oobY'    88    88V8o 88 88
88`8b      88    88 V8o88 88  ooo
88 `88.   .88.   88  V888 88. ~8~
88   YD Y888888P VP   V8P  Y888P


*/




#[derive(Clone)]
pub struct Ring<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    size: T,
    elements: Vec<RingElement<T>>,
    buffer: RingElement<T>,
}

impl<T> Ring<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    pub fn new_with_size(size: T) -> Self {
        // buffer in case all min-sizes are reached.
        // the ring is always filled out!
        Ring {
            size,
            elements: Vec::new(),
            buffer: RingElement::new()
        }
    }
    pub fn new() -> Self {
        Ring::new_with_size(T::from(1).unwrap())
    }

    pub fn get(&self, index: usize) -> T {
        if index == self.elements.len() {
            T::zero()
        } else {
            self.elements[index].get_size(self.size)
        }
    }

    pub fn get_sum(&self, mut index: usize) -> T {
        let mut res = T::zero();
        if index == 0 { return res; }
        if index > self.elements.len() {
            index = self.elements.len();
        }

        for i in 0..index {
            res = res + self.get(i);
        }
        res
    }

    pub fn push(&mut self, element: RingElement<T>) {
        let n = self.elements.len();
        self.insert(n, element);
    }

    pub fn insert(&mut self, index: usize, element: RingElement<T>) {
        // check if it fits at all
        let mut min = T::zero();

        for el in &self.elements {
            min = min + el.get_min(self.size);
        }
        if DEBUG { println!("minimal size for all elements {:?}", min);}
        let rem = self.size - min;

        if DEBUG {
            let elmin = element.get_min(self.size);

            if rem < elmin {
                println!("ERROR: Cannot insert element into Ring: no size left. Ignoring insert...");
                println!("       size left: {:?}, min size of element: {:?}", rem, elmin);
            }

            if rem < element.size.to_value(self.size) {
                println!("shrinking element to fit in leftover space...")
            }
        }

        // add element since possible
        //rem = rem - element.min.to_value(self.size);

        if index >= self.elements.len() {
            self.elements.push(element);
        } else {
            self.elements.insert(index, element);
        }

        self.rescale_elements();
    }

    pub fn pop(&mut self) -> Option<RingElement<T>> {
        let el = self.elements.pop();
        self.rescale_elements();
        el
    }

    pub fn remove(&mut self, index: usize) -> RingElement<T> {
        let el = self.elements.remove(index);
        self.rescale_elements();
        el
    }

    pub fn resize(&mut self, size: T) {
        self.size = size;
        self.rescale_elements();
    }

    fn rescale_elements(&mut self) {
        if DEBUG { println!("Ring: rescale_elements...");}
        let num = self.elements.len();

        // shrink all elements before expanding again
        self.buffer.shrink_to_min(self.size);
        let mut min = T::from(0).unwrap();
        for el in &mut self.elements {
            el.shrink_to_min(self.size);
            if DEBUG { println!("{:?}", el); }
            min = min + el.get_min(self.size);
        }
        let mut rem = self.size - min;

        // gradually expand as long as there is space left
        if DEBUG { println!("starting remnant {:?}", rem);}
        let mut num_growable = num;

        let mut checked = Vec::with_capacity(num);
        for _ in 0..num { checked.push(false); }

        while rem > T::from(0).unwrap() {
            if DEBUG { println!(" --- loop --- {}", num_growable);}

            // calculate remnant space in this iteration and the amount to grow
            // TODO implement weights for each element
            let rem_grow = T::from( rem.to_f64().unwrap() / (num_growable as f64)).unwrap();

            // consider rem_grow = 0.5 for int, so actually rem_grow = 0, but still space left
            let mut sub_one = rem - rem_grow * T::from(num_growable).unwrap();

            // grow
            let mut el_size = T::zero();
            for k in 0..num {
                let el = &mut self.elements[k];
                if !checked[k] {
                    if sub_one > T::zero() {
                        el.grow(self.size, rem_grow + T::one());
                        sub_one = sub_one - T::one();
                    } else {
                        el.grow(self.size, rem_grow);
                    }
                }
                el_size = el_size + el.get_size(self.size);
            }
            rem = self.size - el_size;
            if DEBUG { println!("    remnant {:?}", rem);}

            // all max_sizes reached?
            let mut all_max = true;
            num_growable = num;
            for k in 0..num {
                let el = &mut self.elements[k];
                if !checked[k] && !el.is_at_max(self.size) {
                    all_max = false;
                } else {
                    checked[k] = true;
                    num_growable -= 1;
                }
            }
            if all_max {
                if DEBUG { println!("every element at max. Filling buffer...");}
                self.buffer.size = RingElementSize::Absolute(rem);
                break
            }
        }
    }
}

#[allow(unused_must_use)]
impl<T> Debug for Ring<T> where T: Num + NumCast + PartialOrd + Copy + Debug {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Ring: size {:?}", self.size);
        writeln!(f, "    buffer: size {:?}", self.buffer.size);
        for n in 0..self.elements.len() {
            let min = self.elements[n].min.to_value(self.size);
            let size = self.elements[n].size.to_value(self.size);
            let max = self.elements[n].max.to_value(self.size);

            writeln!(f, "    min: {:?}, size: {:?}, max: {:?}", min, size, max);
        }
        writeln!(f, "")
    }
}




















/*
d8888b. d888888b .88b  d88.
88  `8D   `88'   88'YbdP`88
88   88    88    88  88  88
88   88    88    88  88  88
88  .8D   .88.   88  88  88
Y8888D' Y888888P YP  YP  YP


*/



#[derive(Debug, Copy, Clone)]
pub enum Dim {
    Absolute(i32),
    Relative(f64),
}





















/*
db    db d88888b  .o88b. .d888b.
88    88 88'     d8P  Y8 VP  `8D
Y8    8P 88ooooo 8P         odD'
`8b  d8' 88~~~~~ 8b       .88'
 `8bd8'  88.     Y8b  d8 j88.
   YP    Y88888P  `Y88P' 888888D


*/



#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    pub x: T, pub y: T
}

impl<T> Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    pub fn zero() -> Vec2<T> {
        Vec2{
            x: T::from(0).unwrap(), y: T::from(0).unwrap(),
        }
    }

    pub fn el_mul(self, other: Vec2<T>) -> Vec2<T> {
        Vec2{
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl<T> Add for Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    type Output = Vec2<T>;

    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    type Output = Vec2<T>;

    fn sub(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Div<T> for Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Vec2<T> {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> Mul<T> for Vec2<T> where T: Num + NumCast + PartialOrd + Copy {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Vec2<T> {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

















/*
d88888b d8888b.  .d8b.  .88b  d88. d88888b
88'     88  `8D d8' `8b 88'YbdP`88 88'
88ooo   88oobY' 88ooo88 88  88  88 88ooooo
88~~~   88`8b   88~~~88 88  88  88 88~~~~~
88      88 `88. 88   88 88  88  88 88.
YP      88   YD YP   YP YP  YP  YP Y88888P


*/



#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Frame<T> where  T: Num + NumCast + PartialOrd + Copy {
    pub p0: Vec2<T>, pub p1: Vec2<T>,
}
impl<T> Frame<T> where T: Num + NumCast + PartialOrd + Copy {
    pub fn new_with_size(width: T, height: T) -> Self {
        Frame{
            p0: Vec2{x: T::zero(), y: T::zero()},
            p1: Vec2{x: width, y: height}
        }
    }

    pub fn new() -> Self {
        let one = T::one();
        Frame::new_with_size(one, one)
    }

    pub fn width(&self) -> T {
        self.p1.x - self.p0.x
    }
    pub fn height(&self) -> T {
        self.p1.y - self.p0.y
    }
    pub fn max_dim(&self) -> T {
        if self.width() >= self.height() { return self.width() } else { return self.height() }
    }
    pub fn min_dim(&self) -> T {
        if self.width() <= self.height() { return self.width() } else { return self.height() }
    }
    pub fn center(&self) -> Vec2<T> {
        (self.p1 + self.p0) / (T::from(2).unwrap())
    }
    pub fn size(&self) -> Vec2<T> {
        self.p1 - self.p0
    }

    pub fn inside(&self, x: T, y: T) -> bool {
        x >= self.p0.x && x <= self.p1.x && y >= self.p0.y && y<= self.p1.y
    }
    //pub fn center_left(&self) -> Vec2<T>
}















/*
 d888b  d8888b.  .d8b.  d8888b. db   db d888888b  .o88b.
88' Y8b 88  `8D d8' `8b 88  `8D 88   88   `88'   d8P  Y8
88      88oobY' 88ooo88 88oodD' 88ooo88    88    8P
88  ooo 88`8b   88~~~88 88~~~   88~~~88    88    8b
88. ~8~ 88 `88. 88   88 88      88   88   .88.   Y8b  d8
 Y888P  88   YD YP   YP 88      YP   YP Y888888P  `Y88P'


*/


#[derive(Debug, Clone)]
pub enum Graphic {
    Color(conrod::Color),
    Texture(Texture),
    None,
}









/*
d888888b d88888b db    db d888888b db    db d8888b. d88888b
`~~88~~' 88'     `8b  d8' `~~88~~' 88    88 88  `8D 88'
   88    88ooooo  `8bd8'     88    88    88 88oobY' 88ooooo
   88    88~~~~~  .dPYb.     88    88    88 88`8b   88~~~~~
   88    88.     .8P  Y8.    88    88b  d88 88 `88. 88.
   YP    Y88888P YP    YP    YP    ~Y8888P' 88   YD Y88888P


*/

#[derive(Debug, Copy, Clone)]
pub enum TextureMode {
    Stretch,
    FitWidth,
    FitHeight,
    FitMin,
    FitMax,
    Tile,
}

#[derive(Debug, Clone)]
pub struct Texture {
    id: String,
    cut: Option<Frame<u32>>,
    mode: TextureMode,
}

impl Texture {
    pub fn new(id: String) -> Self {
        Texture {
            id,
            cut: None,
            mode: TextureMode::Stretch,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn set_cut(&mut self, cut: Frame<u32>) {
        self.cut = Some(cut);
    }

    pub fn with_cut(mut self, cut: Frame<u32>) -> Self {
        self.cut = Some(cut);
        self
    }

    pub fn set_mode(&mut self, mode: TextureMode) {
        self.mode = mode;
    }

    pub fn with_mode(mut self, mode: TextureMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn get_cut(&self, w: u32,h: u32, img_w: u32, img_h: u32) -> Rect {
        //println!("get_cut() {} {} {} {}", w, h, img_w, img_h);

        // TODO implement pre-defined texture cut //////////////////////////////////////////////////
        //let mut tex_cut = match self.cut {
        //    Some(c) => c,
        //    None => Frame::new()
        //};
//
        let ratio = w as f64 / h as f64;
        //let (min, max) = if w > h {(h,w)} else {(w,h)};
//
        //let img_ratio = img_w as f64 / img_h as f64;
        //let (img_min, img_max) = if img_w > img_h {(img_h, img_w)} else {(img_w, img_h)};

        match self.mode {
            TextureMode::FitHeight => {
                Rect::from_corners([0.0,0.0], [ratio * img_w as f64, img_h as f64])
            },
            TextureMode::FitWidth => {
                Rect::from_corners([0.0,0.0], [img_w as f64, img_h as f64 / ratio])
            },
            TextureMode::FitMax => {
                if ratio > 1.0 {
                    Rect::from_corners([0.0,0.0], [img_w as f64, img_h as f64 / ratio])
                } else {
                    Rect::from_corners([0.0,0.0], [ratio * img_w as f64, img_h as f64])
                }
            },
            TextureMode::FitMin => {
                if ratio < 1.0 {
                    Rect::from_corners([0.0,0.0], [img_w as f64, img_h as f64 / ratio])
                } else {
                    Rect::from_corners([0.0,0.0], [ratio * img_w as f64, img_h as f64])
                }
            },
            _ => {
                println!("get_cut() Stretch");
                Rect::from_corners([0.0, 0.0], [img_w as f64, img_h as f64])
            },
        }
    }
}




















/*
                   d888888b d8888b.  .d8b.  d888888b d888888b .d8888.
                   `~~88~~' 88  `8D d8' `8b   `88'   `~~88~~' 88'  YP
                      88    88oobY' 88ooo88    88       88    `8bo.
C8888D C8888D         88    88`8b   88~~~88    88       88      `Y8b.      C8888D C8888D
                      88    88 `88. 88   88   .88.      88    db   8D
                      YP    88   YD YP   YP Y888888P    YP    `8888Y'


*/




use std::i32;

pub trait Element {
    fn setup(&mut self, ui: &mut conrod::Ui);
    fn is_setup(&self) -> bool;

    fn set_parent_widget(&mut self, parent: conrod::widget::id::Id);
    fn set_floating(&mut self, floating: bool);

    fn stop(&mut self) {}
    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources);

    fn get_frame(&self) -> Frame<i32>;
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>);

    fn set_min_size(&mut self, size: Vec2<i32>);
    fn get_min_size(&self) -> Vec2<i32>;
    fn set_max_size(&mut self, size: Vec2<i32>);
    fn get_max_size(&self) -> Vec2<i32>;

    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool);
}

pub trait Labelable {
    fn with_label(self, label: String) -> Box<Self>;
    fn set_label(&mut self, label: String);

    fn with_font(self, font: String) -> Box<Self>;
    fn set_font(&mut self, font: String);
}

pub trait Colorable {
    fn with_color(self, color: conrod::Color) -> Box<Self>;
    fn set_color(&mut self, color: conrod::Color);
}


pub trait Graphicable {
    fn with_graphic(self, fg: Graphic) -> Box<Self>;
    fn set_graphic(&mut self, fg: Graphic);
}



















/*
db   d8b   db d888888b d8b   db d8888b.  .d88b.  db   d8b   db
88   I8I   88   `88'   888o  88 88  `8D .8P  Y8. 88   I8I   88
88   I8I   88    88    88V8o 88 88   88 88    88 88   I8I   88
Y8   I8I   88    88    88 V8o88 88   88 88    88 Y8   I8I   88
`8b d8'8b d8'   .88.   88  V888 88  .8D `8b  d8' `8b d8'8b d8'
 `8b8' `8d8'  Y888888P VP   V8P Y8888D'  `Y88P'   `8b8' `8d8'


*/

pub struct WindowRessources {
    fonts: HashMap<String, conrod::text::font::Id>,
    image_map: conrod::image::Map<glium::texture::Texture2d>,
    images: HashMap<String, (u32, u32, conrod::image::Id)>
}
impl WindowRessources {
    pub fn new() -> Self {
        WindowRessources {
            fonts: HashMap::new(),
            image_map: conrod::image::Map::new(),
            images: HashMap::new(),
        }
    }

    pub fn add_font(&mut self,  ui: &mut conrod::Ui, id: String, path: &Path) {
        self.fonts.insert(
            id,
            ui.fonts.insert_from_file(path).unwrap()
        );
        println!("font loaded into windowressource.");
        println!("self.fonts {:?}", self.fonts);
    }

    pub fn font(&self, id: &String) -> Option<&conrod::text::font::Id> {
        self.fonts.get(id)
    }

    pub fn image(&self, id: &String) -> Option<&(u32,u32,conrod::image::Id)> {
        self.images.get(id)
    }

    pub fn add_image(&mut self, display: &glium::Display, id: String, path: &Path) {
        let img = Self::load_image(display, path);
        let (w,h) = (img.get_width(), img.get_height().unwrap());
        self.images.insert(
            id,
            (
                w, h,
                self.image_map.insert(
                    Self::load_image(display, path)
                )
            )
        );
        println!("image loaded into windowressource.");
        println!("self.images {:?}", self.images);
    }

    // ? from conrod-example "image_button.rs"
    // Load an image from our assets folder as a texture we can draw to the screen.
    fn load_image<P>(display: &glium::Display, path: P) -> glium::texture::Texture2d
        where P: AsRef<Path>,
    {
        let path = path.as_ref();
        let rgba_image = image::open(&Path::new(&path)).unwrap().to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
        texture
    }
}


widget_ids!(
    struct WindowIds {
        window
    }
);


pub struct Window {
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    renderer: conrod::backend::glium::Renderer,
    //image_map: conrod::image::Map<glium::texture::Texture2d>,
    ui: conrod::Ui,

    element: Option<Box<Element>>,
    receivers: Vec<Receiver<ActionMsg>>,
    senders: Vec<Sender<ActionMsg>>,

    selfsender: Sender<ActionMsg>,

    ids: Option<WindowIds>,
    ressources: WindowRessources,
}

impl Window {

    fn setup(&mut self) {
        let ids = WindowIds::new(self.ui.widget_id_generator());
        println!("setup(): starting...");
        if let Some(ref mut el) = self.element {
            el.set_parent_widget(ids.window);
            el.setup(&mut self.ui);
            println!("setup(): element setup.");
        }
        self.ids = Some(ids);
        println!("setup(): done.");
    }



    pub fn add_font(&mut self, id: String, path: &Path) {
        self.ressources.add_font(
            &mut self.ui,
            id, 
            path
        );
    }

    pub fn add_image(&mut self, id: String, path: &Path) {
        self.ressources.add_image(
            &mut self.display, 
            id, 
            path
        );
    }




    pub fn add_element(&mut self, element: Box<Element>) {
        self.element = Some(element);
    }

    pub fn add_receiver(&mut self, receiver: Receiver<ActionMsg>) {
        self.receivers.push(receiver);
    }

    pub fn add_sender(&mut self, sender: Sender<ActionMsg>) {
        self.senders.push(sender);
    }

    fn send(&mut self, msg: ActionMsgData) {
        for sender in &mut self.senders {
            let tmp = msg.clone();
            let _ = sender.send(ActionMsg{
                sender_id: "Window".to_string(),
                msg: tmp
            });
        }
        
        let _ = self.selfsender.send(ActionMsg{
            sender_id: "Window".to_string(),
            msg
        });
    }


    pub fn new(title: String, width: u32, height: u32) -> Self {
        // build window
        let events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(8);
        let display = glium::Display::new(
            window, context, &events_loop
        ).unwrap();



        // create conrod ui
        let mut ui =  conrod::UiBuilder::new(
            [width as f64, height as f64]
        ).build();


        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();

        let mut ressources = WindowRessources::new();
        ressources.add_font(
            &mut ui,
            "NotoSans-Regular".to_string(), 
            &assets.join("fonts/NotoSans/NotoSans-Regular.ttf")
        );

        // Add a `Font` to the `Ui`'s `font::Map` from file.
        //let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        //let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        //ui.fonts.insert_from_file(font_path).unwrap();

        // Add a font to the ui's font::map from file
        /*const FONT_PATH: &'static str =
            concat!(env!("CARGO_MANIFEST_DIR"),
                "\\assets\\fonts\\NotoSans\\NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(FONT_PATH).unwrap();*/

        // connect conrod::render::Primitives to glium Surface
        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // image mapping, here: none
        //let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        let (selfsender, selfreceiver): (Sender<ActionMsg>, Receiver<ActionMsg>)
            = mpsc::channel();


        Window {
            events_loop,
            display,
            renderer,
            //image_map,
            ui,
            element: None,
            receivers: vec![selfreceiver],
            senders: Vec::new(),
            selfsender,
            ids: None,
            ressources
        }
    }

    pub fn run(&mut self) {
        self.run_with_fps(-1f64);
    }

    pub fn run_with_fps(&mut self, fps: f64) {

        self.setup();

        println!("run(): window setup.");

        let dt_ns = (1.0e9/fps) as u64;

        // events
        let mut events = Vec::new();
        let mut t0 = time::precise_time_ns();

        let mut window_frame = Frame::new();


        // mouse helper
        struct MouseDrag {
            pub current: (f64,f64),
            pub left: bool,
            pub right: bool,
            pub middle: bool,
            start_left: (f64,f64),
            start_right: (f64,f64),
            start_middle: (f64, f64),
        }
        impl MouseDrag {
            pub fn update(&mut self, x: f64, y: f64) {
                self.current = (x,y);
            }

            pub fn start_left(&mut self) {
                self.left = true;
                self.start_left = self.current;
            }
            pub fn start_right(&mut self) {
                self.right = true;
                self.start_right = self.current;
            }
            pub fn start_middle(&mut self) {
                self.middle = true;
                self.start_middle = self.current;
            }

            pub fn stop_left(&mut self) {
                self.left = false;
            }
            pub fn stop_right(&mut self) {
                self.right = false;
            }
            pub fn stop_middle(&mut self) {
                self.middle = false;
            }
            
            pub fn get_left(&self) -> ActionMsgData {
                let (x,y) = self.current;
                let (xx,yy) = (x-self.start_left.0, y-self.start_left.1);
                ActionMsgData::MouseDragLeft(xx,yy)
            }
            pub fn get_right(&self) -> ActionMsgData {
                let (x,y) = self.current;
                let (xx,yy) = (x-self.start_right.0, y-self.start_right.1);
                ActionMsgData::MouseDragRight(xx,yy)
            }
            pub fn get_middle(&self) -> ActionMsgData {
                let (x,y) = self.current;
                let (xx,yy) = (x-self.start_middle.0, y-self.start_middle.1);
                ActionMsgData::MouseDragMiddle(xx,yy)
            }
        }

        let mut mouse_drag = MouseDrag{
            current: (0.0,0.0),
            left: false, right: false, middle: false,
            start_left: (0.0,0.0),
            start_right: (0.0,0.0),
            start_middle: (0.0,0.0)
        };


        let mut window_height = 0;


        //println!("loop is starting ...............................");
        'render: loop {
            events.clear();

            // get new events after last frame
            self.events_loop.poll_events(|event| {
                events.push(event);
            });


            let mut update = false;
            let mut resized = false;

            // process events
            for event in events.drain(..) {

                use conrod::glium::glutin::{Event, WindowEvent, KeyboardInput, VirtualKeyCode,
                MouseButton, ElementState};

                match event.clone() {
                    Event::WindowEvent { event, .. } => {
                        match event {
                            WindowEvent::MouseInput {
                                state,
                                button,
                                ..
                            } => {
                                match state {
                                    ElementState::Pressed => {
                                        let (x,y) = mouse_drag.current;
                                        match button {
                                            MouseButton::Left => {
                                                mouse_drag.start_left();
                                                self.send(ActionMsgData::MousePressLeft(x,y));
                                            },
                                            MouseButton::Right => {
                                                mouse_drag.start_right();
                                                self.send(ActionMsgData::MousePressRight(x,y));
                                            },
                                            MouseButton::Middle => {
                                                mouse_drag.start_middle();
                                                self.send(ActionMsgData::MousePressMiddle(x,y));
                                            },
                                            _ => (),
                                        }
                                    },
                                    ElementState::Released => {
                                        let (x,y) = mouse_drag.current;
                                        match button {
                                            MouseButton::Left => {
                                                mouse_drag.stop_left();
                                                self.send(ActionMsgData::MouseReleaseLeft(x,y));
                                            },
                                            MouseButton::Right => {
                                                mouse_drag.stop_right();
                                                self.send(ActionMsgData::MouseReleaseRight(x,y));
                                            },
                                            MouseButton::Middle => {
                                                mouse_drag.stop_middle();
                                                self.send(ActionMsgData::MouseReleaseMiddle(x,y));
                                            },
                                            _ => (),
                                        }
                                    },
                                    _ => (),
                                }
                            }
                            WindowEvent::CursorMoved {
                                position: (x,y),
                                ..
                            } => {
                                //println!("mouse moved {}, {}",x,y);
                                self.send(ActionMsgData::Mouse(x,window_height as f64 - y));

                                mouse_drag.update(x,window_height as f64 - y);
                                if mouse_drag.left {
                                    self.send(mouse_drag.get_left());
                                }
                                if mouse_drag.right {
                                    self.send(mouse_drag.get_right());
                                }
                                if mouse_drag.middle {
                                    self.send(mouse_drag.get_middle());
                                }
                            },
                            WindowEvent::Closed |
                            WindowEvent::KeyboardInput {
                                input: KeyboardInput{
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                            } => {
                                self.send(ActionMsgData::Exit);
                                use std::thread;
                                use std::time::Duration;
                                thread::sleep(Duration::from_millis(1000));
                                break 'render
                            },
                            WindowEvent::Resized(mut w, mut h) => {

                                window_height = h;

                                // TODO is this limit necessary?
                                if let Some(ref mut el) = self.element {
                                    let min = el.get_min_size();
                                    let max = el.get_max_size();

                                    if w > max.x as u32 { w = max.x as u32; }
                                    if w < min.x as u32 { w = min.x as u32; }
                                    if h > max.y as u32 { h = max.y as u32; }
                                    if h < min.y as u32 { h = min.y as u32; }

                                    window_frame = Frame::new_with_size(w as i32,h as i32);
                                    resized = true;
                                }
                            }
                            _ => (),
                        }
                    },
                    _ => (),
                };

                // convert winit event to conrod input
                let input = match conrod::backend::winit::convert_event(event, &self.display) {
                    None => continue,
                    Some(input) => input,
                };

                // handle input
                self.ui.handle_event(input);

                update = true;
            }

            // check for fps forced update
            if fps > 0.0 {
                let time_diff = time::precise_time_ns() - t0;
                if time_diff >= dt_ns {
                    self.ui.needs_redraw();
                    t0 = time::precise_time_ns();
                    update = true;
                }
            }

            // check if msgs have to be processed and transmit through chain
            for receiver in &self.receivers {
                'receive: loop {
                    match receiver.try_recv() {
                        Ok(msg) => {
                            if DEBUG { println!("message received: {:?}", msg); }
                            if let Some(ref mut el) = self.element {
                                el.transmit_msg(msg.clone(), false);
                            }
                            update = true;
                            for sender in &mut self.senders {
                                let _ = sender.send(msg.clone());
                            }

                            match msg.msg {
                                ActionMsgData::Update => self.ui.needs_redraw(),
                                _ => ()
                            }
                        },
                        _ => break 'receive
                    }
                }
            }

            if let Some(ref mut el) = self.element {
                if !el.is_setup() {
                    el.setup(&mut self.ui);
                    update = true;
                }
            }

            if resized {
                if let Some(ref mut el) = self.element {
                    el.set_frame(window_frame, window_frame.center());
                }
            }

            if update {
                let ui = &mut self.ui.set_widgets();
                let res = &self.ressources;
                if DEBUG { println!("run() start building...");}

                if let Some(ref ids) = self.ids {
                    use conrod::{widget, Widget, Colorable};
                    widget::canvas::Canvas::new()
                        .rgba(0.0,0.0,0.0,0.0)
                        .set(ids.window, ui);
                }

                if let Some(ref mut el) = self.element {
                    //el.set_frame(window_frame, window_frame.center());
                    el.build_window(ui, res);
                    if DEBUG { println!("run() element build...");}
                }
                if DEBUG { println!("run() all elements build.");}
            }

            // draw ui if changed
            if let Some(primitives) = self.ui.draw_if_changed() {
                if DEBUG { println!("run() preparing target for drawing...");}
                self.renderer.fill(&self.display, primitives, &self.ressources.image_map);
                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);

                if DEBUG { println!("run() drawing..."); }
                self.renderer.draw(&self.display, &mut target, &self.ressources.image_map).unwrap();
                target.finish().unwrap();
                if DEBUG { println!("run() loop finished."); }
            }
        }

        if let Some(ref mut el) = self.element {
            el.stop();
        }
    }
}

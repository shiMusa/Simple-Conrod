



use conrod;

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
    parent: Option<conrod::widget::id::Id>,
    is_setup: bool,
    frame: Frame<i32>,
    window_center: Vec2<i32>,

    min_size: Vec2<i32>,
    max_size: Vec2<i32>,
}
impl Empty {
    pub fn new() -> Box<Self> {
        Box::new(Empty{
            parent: None,
            is_setup: false,
            frame: Frame::new(),
            window_center: Vec2::zero(),

            min_size: Vec2::zero(),
            max_size: Vec2 {x: i32::MAX, y: i32::MAX},
        })
    }
}
impl Element for Empty {
    fn setup(&mut self, _ui: &mut conrod::Ui) { self.is_setup = true }
    fn is_setup(&self) -> bool { self.is_setup }

    fn set_parent_widget(&mut self, parent: conrod::widget::id::Id) {
        self.parent = Some(parent);
    }
    fn set_floating(&mut self, _floating: bool) {}

    fn build_window(&self, _ui: &mut conrod::UiCell, _ressources: &WindowRessources) {}

    fn get_frame(&self) -> Frame<i32> { self.frame }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.frame = frame;
        self.window_center = window_center;

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

    fn transmit_msg(&mut self, _msg: ActionMsg, _stop: bool) {}
}










/*
d8888b. db       .d8b.  d8b   db d88888b
88  `8D 88      d8' `8b 888o  88 88'
88oodD' 88      88ooo88 88V8o 88 88ooooo
88~~~   88      88~~~88 88 V8o88 88~~~~~
88      88booo. 88   88 88  V888 88.
88      Y88888P YP   YP VP   V8P Y88888P


*/




widget_ids!(
    #[derive(Clone)]
    struct PlaneIds {
        plane,
    }
);

#[derive(Clone)]
pub struct Plane {
    ids: Option<PlaneIds>,
    parent: Option<conrod::widget::id::Id>,
    floating: bool,

    is_setup: bool,
    global_center: Vec2<i32>,
    frame: Frame<i32>,

    graphic: Graphic,

    min_size: Vec2<i32>,
    max_size: Vec2<i32>,
}

impl Plane {
    pub fn new(graphic: Graphic) -> Box<Self> {
        use std::i32;
        Box::new(Plane {
            ids: None,
            parent: None,
            floating: false,

            is_setup: false,
            global_center: Vec2::zero(),
            frame: Frame::new(),

            graphic,
            min_size: Vec2::zero(),
            max_size: Vec2 {x: i32::MAX, y: i32::MAX},
        })
    }

    fn build_textured(
        &self, 
        _ui: &mut conrod::UiCell, 
        _ressources: &WindowRessources, 
        texture: (u32,u32,conrod::image::Id),
        texture_properties: &Texture
    ) {
        use conrod::{Widget, widget, Positionable, Sizeable};

        if DEBUG { println!("building textured plane with image id {:?}", texture);}
        let c = self.frame.center()-self.global_center;

        if DEBUG { println!("creating plane image...");}
        let mut img = widget::primitive::image::Image::new(texture.2)
            .source_rectangle(texture_properties.get_cut(
                self.frame.width() as u32, self.frame.height() as u32, texture.0, texture.1
            ))
            .floating(self.floating)
            .x_y(c.x as f64, c.y as f64)
            .w_h(self.frame.width() as f64,self.frame.height() as f64);
        if let Some(parent) = self.parent {
            img = img.parent(parent);
        }
        if let Some(ref ids) = self.ids {
            img.set(ids.plane, _ui);
        }
        if DEBUG { println!("Plane build.");}
    }

    fn build_flat(&self, ui: &mut conrod::UiCell, _ressources: &WindowRessources, color: conrod::Color) {
        use conrod::{Positionable, Widget};

        if DEBUG { println!("building flat plane");}
        if let Some(ref ids) = self.ids {
            let c = self.frame.center()-self.global_center;

            if DEBUG { println!("creating plane color...");}
            let mut rect = conrod::widget::Rectangle::fill_with(
                [self.frame.width() as f64, self.frame.height() as f64],
                color
            ).x_y(c.x as f64, c.y as f64);
            if let Some(parent) = self.parent {
                rect = rect.parent(parent);
            }
            rect.set(ids.plane, ui);
        }
        
        if DEBUG { println!("Plane build.");}
    }
}

impl Graphicable for Plane {
    fn with_graphic(mut self, fg: Graphic) -> Box<Self> {
        self.graphic = fg;
        Box::new(self)
    }
    fn set_graphic(&mut self, fg: Graphic) {
        self.graphic = fg;
    }
}

impl Element for Plane {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.ids = Some(PlaneIds::new(ui.widget_id_generator()));
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool { self.is_setup }

    fn set_parent_widget(&mut self, parent: conrod::widget::id::Id) {
        self.parent = Some(parent);
    }
    fn set_floating(&mut self, floating: bool) {
        self.floating = floating;
    }

    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        match self.graphic {
            Graphic::Texture(ref texture) => {
                if let Some(tex) = ressources.image(&texture.get_id()) {
                    self.build_textured(ui, ressources, *tex, &texture);
                    return;
                };
            },
            Graphic::Color(color) => {
                self.build_flat(ui, ressources, color)
            },
            Graphic::None => ()
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.global_center = window_center;
        self.frame = frame;
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


    fn transmit_msg(&mut self, _msg: ActionMsg, _stop: bool) { }
}








/*
d888888b d88888b db    db d888888b
`~~88~~' 88'     `8b  d8' `~~88~~'
   88    88ooooo  `8bd8'     88
   88    88~~~~~  .dPYb.     88
   88    88.     .8P  Y8.    88
   YP    Y88888P YP    YP    YP


*/







widget_ids!(
    #[derive(Clone)]
    struct LabelIds {
        text,
    }
);

#[derive(Clone)]
pub struct Text {
    font: Font,

    is_setup: bool,
    frame: Frame<i32>,
    global_center: Vec2<i32>,
    min_size: Vec2<i32>,
    max_size: Vec2<i32>,


    ids: Option<LabelIds>,
    parent: Option<conrod::widget::id::Id>,
    floating: bool,
}

impl Text {
    pub fn new(font: Font) -> Box<Self> {
        Box::new(Text {
            font,
            is_setup: false,
            frame: Frame::new(),
            global_center: Vec2::zero(),
            min_size: Vec2::zero(),
            max_size: Vec2 {x: i32::MAX, y: i32::MAX},

            ids: None,
            parent: None,
            floating: false,
        })
    }
}

impl Element for Text {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.ids = Some(LabelIds::new(ui.widget_id_generator()));
        self.is_setup = true;
        if DEBUG { println!("Label --- setup()"); }
    }
    fn is_setup(&self) -> bool { self.is_setup }

    fn set_parent_widget(&mut self, parent: conrod::widget::id::Id) {
        self.parent = Some(parent);
    }

    fn set_floating(&mut self, floating: bool) {
        self.floating = floating;
    }

    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        use conrod::{widget, Positionable, Colorable, Widget};

        if let Some(ref ids) = self.ids {
            let c = self.frame.center() - self.global_center;

            let text = self.font.get_text();
            let mut label = widget::Text::new(&text)
                .x_y(c.x as f64, c.y as f64)
                .color(self.font.get_color())
                .font_size(self.font.get_size())
                .floating(self.floating);

            let fnt = ressources.font(&self.font.get_font_id());
            if let Some(fnt) = fnt {
                label = label.font_id(*fnt);
            }

            if let Some(parent) = self.parent {
                label = label.parent(parent);
            }

            label.set(ids.text, ui);
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.frame = frame;
        self.global_center = window_center;
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


    fn transmit_msg(&mut self, _msg: ActionMsg, _stop: bool){}
}

impl Labelable for Text {
    fn with_font(mut self, font: Font) -> Box<Self> {
        self.font = font;
        Box::new(self)
    }
    fn set_font(&mut self, font: Font) {
        self.font = font;
        self.is_setup = false;
    }
}

impl Colorable for Text {
    fn with_color(mut self, color: conrod::Color) -> Box<Self> {
        self.font.set_color(color);
        Box::new(self)
    }
    fn set_color(&mut self, color: conrod::Color) {
        self.font.set_color(color);
    }
}



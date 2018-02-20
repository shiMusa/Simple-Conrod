



use conrod;

use elements::{*, action::*};

use std::sync::mpsc::{Sender};





const DEBUG: bool = false;




/*
d8888b. db       .d8b.  d8b   db d88888b
88  `8D 88      d8' `8b 888o  88 88'
88oodD' 88      88ooo88 88V8o 88 88ooooo
88~~~   88      88~~~88 88 V8o88 88~~~~~
88      88booo. 88   88 88  V888 88.
88      Y88888P YP   YP VP   V8P Y88888P


*/

widget_ids!(
    struct PlaneIds {
        plane,
    }
);

pub struct Plane {
    ids: Option<PlaneIds>,

    is_setup: bool,
    global_center: Vec2<i32>,
    frame: Frame<i32>,

    graphic: Graphic
}

impl Plane {
    pub fn new(graphic: Graphic) -> Box<Self> {
        Box::new(Plane {
            ids: None,

            is_setup: false,
            global_center: Vec2::zero(),
            frame: Frame::new(),

            graphic,
        })
    }

    fn build_textured(&self, _ui: &mut conrod::UiCell, _ressources: &WindowRessources, texture: conrod::image::Id) {
        use conrod::{Widget, widget, Positionable, Sizeable};

        if DEBUG { println!("building textured plane with image id {:?}", texture);}
        let c = self.frame.center()-self.global_center;

        if DEBUG { println!("creating plane image...");}
        let img = widget::primitive::image::Image::new(texture)
            .x_y(c.x as f64, c.y as f64)
            .w_h(self.frame.width() as f64,self.frame.height() as f64);
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
            rect.set(ids.plane, ui);
        }
        
        if DEBUG { println!("Plane build.");}
    }
}

impl Foregroundable for Plane {
    fn with_foreground(mut self, fg: Graphic) -> Box<Self> {
        self.graphic = fg;
        Box::new(self)
    }
    fn set_foreground(&mut self, fg: Graphic) {
        self.graphic = fg;
    }
}

impl Element for Plane {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.ids = Some(PlaneIds::new(ui.widget_id_generator()));
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool { self.is_setup }

    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        match self.graphic {
            Graphic::Texture(ref texture) => {
                if let Some(tex) = ressources.image(texture) {
                    self.build_textured(ui, ressources, *tex);
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

    fn transmit_msg(&mut self, _msg: ActionMsg, _stop: bool) { }
}








/*
db       .d8b.  d8888b. d88888b db
88      d8' `8b 88  `8D 88'     88
88      88ooo88 88oooY' 88ooooo 88
88      88~~~88 88~~~b. 88~~~~~ 88
88booo. 88   88 88   8D 88.     88booo.
Y88888P YP   YP Y8888P' Y88888P Y88888P


*/






widget_ids!(
    struct LabelIds {
        label,
        background,
    }
);

pub struct Label {
    text: String,
    font_size: u32,
    font: Option<String>,
    color: conrod::Color,
    background: Graphic,

    is_setup: bool,
    frame: Frame<i32>,
    global_center: Vec2<i32>,

    label_ids: Option<LabelIds>,
}

impl Label {
    pub fn new(text: String) -> Box<Self> {
        Label::new_with_font_size(text, 12)
    }
    pub fn new_with_font_size(text: String, font_size: u32) -> Box<Self> {
        Box::new(Label {
            text,
            font_size,
            font: None,
            color: conrod::color::BLACK,
            background: Graphic::None,
            is_setup: false,
            frame: Frame::new(),
            global_center: Vec2::zero(),
            label_ids: None,
        })
    }
}

impl Element for Label {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.label_ids = Some(LabelIds::new(ui.widget_id_generator()));
        self.is_setup = true;
        if DEBUG { println!("Label --- setup()"); }
    }
    fn is_setup(&self) -> bool { self.is_setup }

    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        use conrod::{widget, Positionable, Colorable, Widget};

        if let Some(ref ids) = self.label_ids {
            let c = self.frame.center() - self.global_center;

            match self.background {
                Graphic::None => (),
                Graphic::Color(color) => {
                    let mut rect = conrod::widget::Rectangle::fill_with(
                        [self.frame.width() as f64, self.frame.height() as f64],
                        color
                    ).x_y(c.x as f64, c.y as f64);
                    rect.set(ids.background, ui);
                },
                // TODO Background texture /////////////////////////////////////////
                _ => (),
            }

            let txt = self.text.to_owned();
            let mut label = widget::Text::new(&txt)
                .x_y(c.x as f64, c.y as f64)
                .color(self.color)
                .font_size(self.font_size);

            if let Some(ref font) = self.font {
                let fnt = ressources.font(font);
                if let Some(fnt) = fnt {
                    label = label.font_id(*fnt);
                }
            }

            label.set(ids.label, ui);
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.frame = frame;
        self.global_center = window_center;
    }

    fn get_min_size(&self) -> Vec2<i32> {
        Vec2::zero()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        Vec2{ x: i32::MAX, y: i32::MAX }
    }

    fn transmit_msg(&mut self, _msg: ActionMsg, _stop: bool){}
}

impl Labelable for Label {
    fn with_label(mut self, label: String) -> Box<Self> {
        self.text = label;
        Box::new(self)
    }
    fn set_label(&mut self, label: String) {
        self.text = label;
    }

    fn with_font(mut self, font: String) -> Box<Self> {
        self.font = Some(font);
        Box::new(self)
    }
    fn set_font(&mut self, font: String) {
        self.font = Some(font);
    }
}

impl Colorable for Label {
    fn with_color(mut self, color: conrod::Color) -> Box<Self> {
        self.color = color;
        Box::new(self)
    }
    fn set_color(&mut self, color: conrod::Color) {
        self.color = color;
    }
}

impl Backgroundable for Label {
    fn with_background(mut self, bg: Graphic) -> Box<Self> {
        self.background = bg;
        Box::new(self)
    }
    fn set_background(&mut self, bg: Graphic) {
        self.background = bg;
    }
}

















/*
d8888b. db    db d888888b d888888b  .d88b.  d8b   db
88  `8D 88    88 `~~88~~' `~~88~~' .8P  Y8. 888o  88
88oooY' 88    88    88       88    88    88 88V8o 88
88~~~b. 88    88    88       88    88    88 88 V8o88
88   8D 88b  d88    88       88    `8b  d8' 88  V888
Y8888P' ~Y8888P'    YP       YP     `Y88P'  VP   V8P


*/


widget_ids!(
    struct ButtonIds {
        button,
    }
);




pub struct Button {
    id: String,
    senders: Vec<Sender<ActionMsg>>,

    //receive_fn: Box<Fn(&mut Element, ActionMsg)>,

    is_setup: bool,
    global_center: Vec2<i32>,
    frame: Frame<i32>,

    button_ids: Option<ButtonIds>,
    click_fn: Box<Fn()>,
    
    foreground: Graphic,

    label: Option<String>,
    font: Option<String>,
}

impl Button {
    pub fn new() -> Box<Self> {
        let fun = Box::new(||{});

        let button = Box::new(Button {
            id: "Button".to_string(),
            senders: Vec::new(),
            //receive_fn: rfun,

            is_setup: false,
            global_center: Vec2::zero(),
            frame: Frame::new(),
            button_ids: None,
            click_fn: fun,
            foreground: Graphic::Color(conrod::color::LIGHT_GREY),
            label: None,
            font: None,
        });

        if DEBUG { println!("{:?}", button); }
        button
    }



    fn build_textured(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources, texture: conrod::image::Id) {
        use conrod::{widget, Positionable, Widget, Sizeable, Labelable, Borderable};

        if DEBUG { println!("building textured button with image id {:?}", texture);}

        if let Some(ref ids) = self.button_ids {
            let c = self.frame.center()-self.global_center;

            if DEBUG { println!("creating button...");}
            let mut button = widget::Button::image(texture)
                .x_y(c.x as f64, c.y as f64)
                .w_h(self.frame.width() as f64,self.frame.height() as f64)
                .border(0f64);

            if DEBUG { println!("    setting label");}
            if let Some(ref label) = self.label {
                button = button.label(&label);
            }

            if DEBUG { println!("    setting font");}
            if let Some(ref font) = self.font {
                let fnt = ressources.font(font);
                if let Some(fnt) = fnt {
                    button = button.label_font_id(*fnt);
                }
            }

            if DEBUG { println!("    setting event");}
            let mut event = button.set(ids.button, ui);

            if DEBUG { println!("    executing click-function");}
            if event.was_clicked() {
                // broadcast click action
                for sender in &self.senders {
                    let _ = sender.send(ActionMsg{
                        sender_id: self.id.clone(),
                        msg: ActionMsgData::Click,
                    });
                }

                // execute custom function
                (self.click_fn)();
            }
        }

        if DEBUG { println!("Button build.");}
    }

    fn build_flat(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources, color: conrod::Color) {
        use conrod::{widget, Positionable, Colorable, Widget, Sizeable, Labelable, Borderable};

        if let Some(ref ids) = self.button_ids {
            let c = self.frame.center()-self.global_center;

            let mut button = widget::Button::new()
                .color(color)
                .x_y(c.x as f64, c.y as f64)
                .w_h(self.frame.width() as f64,self.frame.height() as f64)
                .border(0f64);

            if let Some(ref label) = self.label {
                button = button.label(&label);
            }
            if let Some(ref font) = self.font {
                let fnt = ressources.font(font);
                if let Some(fnt) = fnt {
                    button = button.label_font_id(*fnt);
                }
            }

            let mut event = button.set(ids.button, ui);

            if event.was_clicked() {
                // broadcast click action
                for sender in &self.senders {
                    let _ = sender.send(ActionMsg{
                        sender_id: self.id.clone(),
                        msg: ActionMsgData::Click,
                    });
                }

                // execute custom function
                (self.click_fn)();
            }
        }
    }
}








#[allow(unused_must_use)]
impl Debug for Button {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "(Button {:?}) {} ", self.label, self.id);
        writeln!(f, "    {:?}", self.frame);
        writeln!(f, "    {:?}", self.foreground)
    }
}

impl Foregroundable for Button {
    fn with_foreground(mut self, fg: Graphic) -> Box<Self> {
        self.foreground = fg;
        Box::new(self)
    }
    fn set_foreground(&mut self, fg: Graphic) {
        self.foreground = fg;
    }
}

impl Labelable for Button {
    fn with_label(mut self, label: String) -> Box<Self> {
        self.label = Some(label);
        Box::new(self)
    }
    fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }

    fn with_font(mut self, font: String) -> Box<Self> {
        self.font = Some(font);
        Box::new(self)
    }
    fn set_font(&mut self, font: String) {
        self.font = Some(font);
    }
}


impl Clickable for Button {
    fn with_action_click(mut self, fun: Box<Fn()>) -> Box<Self> {
        self.click_fn = fun;
        Box::new(self)
    }
}


impl ActionSendable for Button {
    fn with_id(mut self, id: String) -> Box<Self> {
        self.id = id;
        Box::new(self)
    }
    fn with_sender(mut self, sender: Sender<ActionMsg>) -> Box<Self> {
        self.senders.push(sender);
        Box::new(self)
    }
}



impl Element for Button {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.button_ids = Some(ButtonIds::new(ui.widget_id_generator()));
        self.is_setup = true;
    }
    fn is_setup(&self) -> bool { self.is_setup }

    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        match self.foreground {
            Graphic::Texture(ref texture) => {
                if let Some(tex) = ressources.image(texture) {
                    self.build_textured(ui, ressources, *tex);
                    return;
                };
            },
            Graphic::Color(color) => {
                self.build_flat(ui, ressources, color)
            },
            Graphic::None => {
                let color = conrod::Color::Rgba(0.0,0.0,0.0,0.0);
                self.build_flat(ui, ressources, color)
            }
         }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>) {
        self.global_center = window_center;
        self.frame = frame;
    }

    fn transmit_msg(&mut self, _msg: ActionMsg, _stop: bool) { }
}

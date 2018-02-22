



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
        let img = widget::primitive::image::Image::new(texture.2)
            .source_rectangle(texture_properties.get_cut(
                self.frame.width() as u32, self.frame.height() as u32, texture.0, texture.1
            ))
            .floating(self.floating)
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
    text: String,
    font_size: u32,
    font: Option<String>,
    color: conrod::Color,

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
    pub fn new(text: String) -> Box<Self> {
        Text::new_with_font_size(text, 12)
    }
    pub fn new_with_font_size(text: String, font_size: u32) -> Box<Self> {
        Box::new(Text {
            text,
            font_size,
            font: None,
            color: conrod::color::BLACK,
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

            let txt = self.text.to_owned();
            let mut label = widget::Text::new(&txt)
                .x_y(c.x as f64, c.y as f64)
                .color(self.color)
                .font_size(self.font_size)
                .floating(self.floating);

            if let Some(ref font) = self.font {
                let fnt = ressources.font(font);
                if let Some(fnt) = fnt {
                    label = label.font_id(*fnt);
                }
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

impl Colorable for Text {
    fn with_color(mut self, color: conrod::Color) -> Box<Self> {
        self.color = color;
        Box::new(self)
    }
    fn set_color(&mut self, color: conrod::Color) {
        self.color = color;
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
    #[derive(Clone)]
    struct ButtonIds {
        button,
    }
);



#[derive(Clone)]
pub struct Button {
    id: String,
    senders: Vec<Sender<ActionMsg>>,

    //receive_fn: Box<Fn(&mut Element, ActionMsg)>,

    is_setup: bool,
    global_center: Vec2<i32>,
    frame: Frame<i32>,
    min_size: Vec2<i32>,
    max_size: Vec2<i32>,

    button_ids: Option<ButtonIds>,
    parent: Option<conrod::widget::id::Id>,
    floating: bool,
    
    foreground: Graphic,

    label: Option<String>,
    font: Option<String>,
}

impl Button {
    pub fn new() -> Box<Self> {
        let button = Box::new(Button {
            id: "Button".to_string(),
            senders: Vec::new(),
            //receive_fn: rfun,

            is_setup: false,
            global_center: Vec2::zero(),
            frame: Frame::new(),
            min_size: Vec2::zero(),
            max_size: Vec2 {x: i32::MAX, y: i32::MAX},

            button_ids: None,
            parent: None,
            floating: false,

            foreground: Graphic::Color(conrod::color::LIGHT_GREY),
            label: None,
            font: None,
        });

        if DEBUG { println!("{:?}", button); }
        button
    }



    fn build_textured(
        &self, 
        ui: &mut conrod::UiCell, 
        ressources: &WindowRessources, 
        texture: (u32,u32,conrod::image::Id),
        // TODO implement texture cut and scaling for button ////////////////////////////////////////
        _texture_properties: &Texture
    ) {
        use conrod::{widget, Positionable, Widget, Sizeable, Labelable, Borderable};

        if DEBUG { println!("building textured button with image id {:?}", texture);}

        if let Some(ref ids) = self.button_ids {
            let c = self.frame.center()-self.global_center;

            if DEBUG { println!("creating button...");}
            let mut button = widget::Button::image(texture.2)
                .x_y(c.x as f64, c.y as f64)
                .w_h(self.frame.width() as f64,self.frame.height() as f64)
                .floating(self.floating)
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
                .floating(self.floating)
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

impl Graphicable for Button {
    fn with_graphic(mut self, fg: Graphic) -> Box<Self> {
        self.foreground = fg;
        Box::new(self)
    }
    fn set_graphic(&mut self, fg: Graphic) {
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

    fn set_parent_widget(&mut self, parent: conrod::widget::id::Id) {
        self.parent = Some(parent);
    }

    fn set_floating(&mut self, floating: bool) {
        self.floating = floating;
    }

    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources) {
        match self.foreground {
            Graphic::Texture(ref texture) => {
                if let Some(tex) = ressources.image(&texture.get_id()) {
                    self.build_textured(ui, ressources, *tex, &texture);
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

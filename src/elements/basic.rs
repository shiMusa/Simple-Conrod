



use conrod;

use elements::*;

use std::sync::mpsc::{Sender};





const DEBUG: bool = true;



















widget_ids!(
    struct LabelIds {
        label,
        background,
    }
);

pub struct Label {
    text: String,
    font_size: u32,
    color: conrod::Color,
    background: Background,

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
            color: conrod::color::BLACK,
            background: Background::None,
            frame: Frame::new(),
            global_center: Vec2::zero(),
            label_ids: None,
        })
    }
}

impl Element for Label {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.label_ids = Some(LabelIds::new(ui.widget_id_generator()));
    }

    fn build_window(&self, ui: &mut conrod::UiCell) {
        use conrod::{widget, Positionable, Colorable, Widget};

        if let Some(ref ids) = self.label_ids {
            let c = self.frame.center() - self.global_center;

            match self.background {
                Background::None => (),
                Background::Color(color) => {
                    let mut rect = conrod::widget::Rectangle::fill_with(
                        [self.frame.width() as f64, self.frame.height() as f64],
                        color
                    ).x_y(c.x as f64, c.y as f64);
                    rect.set(ids.background, ui);
                }
            }

            widget::Text::new(&self.text.to_owned())
                .x_y(c.x as f64, c.y as f64)
                .color(self.color)
                .font_size(self.font_size)
                .set(ids.label, ui);
        }
    }

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>) {
        self.frame = frame;
    }

    fn get_min_size(&self) -> Vec2<i32> {
        Vec2::zero()
    }
    fn get_max_size(&self) -> Vec2<i32> {
        Vec2{ x: i32::MAX, y: i32::MAX }
    }

    fn set_window_center(&mut self, center: Vec2<i32>) {
        self.global_center = center;
    }

    fn process_and_transmit_msg(&mut self, _msg: &ActionMsg){}
}

impl Labelable for Label {
    fn with_label(mut self, label: String) -> Box<Self> {
        self.text = label;
        Box::new(self)
    }
}

impl Colorable for Label {
    fn with_color(mut self, color: conrod::Color) -> Box<Self> {
        self.color = color;
        Box::new(self)
    }
}

impl Backgroundable for Label {
    fn with_background(mut self, bg: Background) -> Box<Self> {
        self.background = bg;
        Box::new(self)
    }
}





















widget_ids!(
    struct ButtonIds {
        button,
    }
);


pub struct Button {
    id: String,
    senders: Vec<Sender<ActionMsg>>,

    global_center: Vec2<i32>,
    frame: Frame<i32>,

    button_ids: Option<ButtonIds>,
    click_fn: Box<Fn()>,
    color: conrod::Color,

    label: Option<String>,
}

impl Button {
    pub fn new() -> Box<Self> {
        let fun = Box::new(||{});

        let button = Box::new(Button {
            id: "Button".to_string(),
            senders: Vec::new(),

            global_center: Vec2::zero(),
            frame: Frame::new(),
            button_ids: None,
            click_fn: fun,
            color: conrod::color::GRAY,
            label: None,
        });

        if DEBUG { println!("{:?}", button); }
        button
    }
}

#[allow(unused_must_use)]
impl Debug for Button {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "(Button {:?}) {} ", self.label, self.id);
        writeln!(f, "    {:?}", self.frame);
        writeln!(f, "    {:?}", self.color)
    }
}

impl Colorable for Button {
    fn with_color(mut self, color: conrod::Color) -> Box<Self> {
        self.color = color;
        Box::new(self)
    }
}

impl Labelable for Button {
    fn with_label(mut self, label: String) -> Box<Self> {
        self.label = Some(label);
        Box::new(self)
    }
}


impl Clickable for Button {
    fn with_action_click(mut self, fun: Box<Fn()>) -> Box<Self> {
        self.click_fn = fun;
        Box::new(self)
    }
}


impl Actionable for Button {
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
    }

    fn build_window(&self, ui: &mut conrod::UiCell) {
        use conrod::{widget, Positionable, Colorable, Widget, Sizeable, Labelable, Borderable};

        if let Some(ref ids) = self.button_ids {
            let c = self.frame.center()-self.global_center;

            let mut button = widget::Button::new()
                .color(self.color)
                .x_y(c.x as f64, c.y as f64)
                .w_h(self.frame.width() as f64,self.frame.height() as f64)
                .border(0f64);

            if let Some(ref label) = self.label {
                button = button.label(&label);
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

    fn get_frame(&self) -> Frame<i32> {
        self.frame
    }
    fn set_frame(&mut self, frame: Frame<i32>) {
        self.frame = frame;
    }

    fn set_window_center(&mut self, center: Vec2<i32>) {
        self.global_center = center;
    }

    fn process_and_transmit_msg(&mut self, _msg: &ActionMsg) {}
}




use conrod;

use elements::*;






widget_ids!(
    struct ButtonIds {
        button,
    }
);







pub struct Button {
    global_center: Vec2<i32>,
    frame: Frame<i32>,

    min_size: Vec2<i32>,
    max_size: Vec2<i32>,

    button_ids: Option<ButtonIds>,
    click_fn: Box<Fn()>,
    color: conrod::Color,

    label: Option<String>,
}

impl Button {
    pub fn new() -> Box<Self> {
        let fun = Box::new(||{});

        use std::i32;
        Box::new(Button {
            global_center: Vec2::zero(),
            frame: Frame::new(100,100),
            min_size: Vec2::zero(),
            max_size: Vec2{x: i32::MAX, y: i32::MAX},
            button_ids: None,
            click_fn: fun,
            color: conrod::color::GRAY,
            label: None,
        })
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


impl Element for Button {
    fn setup(&mut self, ui: &mut conrod::Ui) {
        self.button_ids = Some(ButtonIds::new(ui.widget_id_generator()));
    }

    fn build_window(&self, ui: &mut conrod::UiCell) {
        use conrod::{widget, Positionable, Colorable, Widget, Sizeable, Labelable};

        if let Some(ref ids) = self.button_ids {
            let c = self.frame.center()-self.global_center;

            let mut button = widget::Button::new()
                .color(self.color)
                .x_y(c.x as f64, c.y as f64)
                .w_h(self.frame.width() as f64,self.frame.height() as f64);

            if let Some(ref label) = self.label {
                button = button.label(&label);
            }

            let mut event = button.set(ids.button, ui);

            if event.was_clicked() {
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
}
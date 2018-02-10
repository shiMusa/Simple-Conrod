



use conrod::{self, widget, Positionable, Colorable, Widget, Sizeable};

use elements::*;






widget_ids!(
    struct ButtonIds {
        button,
    }
);







pub struct Button<F> where F: Fn() {
    global_center: Vec2<i32>,
    frame: Frame<i32>,

    button_ids: ButtonIds,
    click_fn: F,
    color: conrod::Color,
}

impl<F> Button<F> where F: Fn() {
    pub fn new(ui: &mut conrod::Ui, click_fn: F) -> Self {
        let button_ids = ButtonIds::new(ui.widget_id_generator());

        Button {
            global_center: Vec2::zero(),
            frame: Frame::new(100,100),
            button_ids,
            click_fn,
            color: conrod::color::GRAY,
        }
    }
}


impl<F> Element for Button<F> where F: Fn() {
    fn build_window(&self, ui: &mut conrod::UiCell) {
        let c = self.frame.center()-self.global_center;
        if widget::Button::new()
            .color(self.color)
            .x_y(c.x as f64, c.y as f64)
            .w_h(self.frame.width() as f64,self.frame.height() as f64)
            .set(self.button_ids.button, ui)
            .was_clicked() {

            (self.click_fn)();
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
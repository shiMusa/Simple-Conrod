
use elements::*;

use conrod::{self, widget, Positionable, Colorable, Widget};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, RwLock};
use time;
use std::thread;
use std;






#[derive(Clone, Copy, PartialOrd, PartialEq)]
pub enum ClockMsg {
    Stop,
}








pub struct ClockCore {
    time: Arc<RwLock<f64>>,
    receiver: Receiver<ClockMsg>,
}

impl ClockCore {
    pub fn new(time: Arc<RwLock<f64>>, receiver: Receiver<ClockMsg>) -> Self {
        ClockCore { time, receiver }
    }

    pub fn run(&self) {
        const FPS: f64 = 120.0;
        const DT_NS: u64 = (1.0e9/FPS) as u64;

        let t0 = time::precise_time_ns();
        let mut old = t0;

        loop {
            match self.receiver.try_recv() {
                Ok(msg) => match msg {
                    ClockMsg::Stop => break,
                    //_ => (),
                },
                Err(_e) => ()
            }

            {
                let t = time::precise_time_ns() - t0;
                let mut linkdat = self.time.write().unwrap();
                *linkdat = (t as f64) * 1e-9;
            }

            let dt = time::precise_time_ns() - old;

            old = time::precise_time_ns();
            if DT_NS > dt {
                let sl = DT_NS - dt;
                thread::sleep(std::time::Duration::from_nanos(sl));
            }
        }
    }
}
















widget_ids!(
    struct ClockIds {
        circle_sec,
        line_sec,
        text_sec,
        circle_min,
        line_min,
        text_min,
        circle_h,
        line_h,
        text_h,
    }
);



pub struct Clock {
    clock_ids: ClockIds,

    time: Arc<RwLock<f64>>,
    sender: Sender<ClockMsg>,

    size: Vec2u32,
    pos : Vec2f64,
}

impl Clock {
    pub fn new(ui: &mut conrod::Ui, time: Arc<RwLock<f64>>, sender: Sender<ClockMsg>) -> Self {
        let clock_ids = ClockIds::new(ui.widget_id_generator());
        Clock { clock_ids, time, sender, size: (100,100), pos: (0.0,0.0) }
    }
}

impl Element for Clock {
    fn stop(&self) {
        let _ = self.sender.send(ClockMsg::Stop);
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {


        let max = if self.size.0 >= self.size.1 { self.size.1 as f64 } else { self.size.0 as f64 };

        let sec_r = 0.075 * max;
        let min_r = 0.05 * max;
        let h_r   = 0.045 * max;

        let sec_l = 0.2 * max;
        let min_l = 0.35 * max;
        let h_l   = 0.45 * max;



        let t;
        {
            t = *self.time.read().unwrap();
        }

        let hours = (t/(60.0*60.0)) as u8;
        let mins = (t/60.0) as u8;
        let secs = t as u8 - 60*mins;

        let style = widget::primitive::shape::Style::outline_styled(
            widget::primitive::line::Style::solid().thickness(5.0)
        );

        let mut rx = (t / (60.0 * 60.0 * 60.0) * 2.0 * std::f64::consts::PI).sin();
        let mut ry = (t / (60.0 * 60.0 * 60.0) * 2.0 * std::f64::consts::PI).cos();
        widget::Circle::styled(h_r, style)
            .x_y(rx * h_l + self.pos.0,ry*h_l + self.pos.1)
            .color(conrod::color::LIGHT_BROWN)
            .set(self.clock_ids.circle_h, ui);

        widget::Line::new([self.pos.0, self.pos.1], [rx*(h_l-h_r) + self.pos.0,ry*(h_l-h_r) + self.pos.1])
            .thickness(5.0)
            .color(conrod::color::LIGHT_BROWN)
            .set(self.clock_ids.line_h, ui);

        widget::Text::new(&format!("{:2}",hours))
            .x_y(rx*h_l + self.pos.0,ry*h_l + self.pos.1)
            .color(conrod::color::LIGHT_BROWN)
            .font_size(48)
            .set(self.clock_ids.text_h, ui);


        rx = (t / (60.0 * 60.0) * 2.0 * std::f64::consts::PI).sin();
        ry = (t / (60.0 * 60.0) * 2.0 * std::f64::consts::PI).cos();
        widget::Circle::styled(min_r, style)
            .x_y(rx*min_l + self.pos.0,ry*min_l + self.pos.1)
            .color(conrod::color::LIGHT_GREEN)
            .set(self.clock_ids.circle_min, ui);

        widget::Line::new([self.pos.0, self.pos.1], [rx*(min_l-min_r) + self.pos.0,ry*(min_l-min_r) + self.pos.1])
            .thickness(5.0)
            .color(conrod::color::LIGHT_GREEN)
            .set(self.clock_ids.line_min, ui);

        widget::Text::new(&format!("{:2}",mins))
            .x_y(rx*min_l + self.pos.0,ry*min_l + self.pos.1)
            .color(conrod::color::LIGHT_GREEN)
            .font_size(48)
            .set(self.clock_ids.text_min, ui);


        rx = (t / 60.0 * 2.0 * std::f64::consts::PI).sin();
        ry = (t / 60.0 * 2.0 * std::f64::consts::PI).cos();
        widget::Circle::styled(sec_r, style)
            .x_y(rx*sec_l + self.pos.0,ry*sec_l + self.pos.1)
            .color(conrod::color::LIGHT_BLUE)
            .set(self.clock_ids.circle_sec, ui);

        widget::Line::new([self.pos.0, self.pos.1], [rx*(sec_l-sec_r) + self.pos.0,ry*(sec_l-sec_r) + self.pos.1])
            .thickness(5.0)
            .color(conrod::color::LIGHT_BLUE)
            .set(self.clock_ids.line_sec, ui);

        widget::Text::new(&format!("{:2}",secs))
            .x_y(rx*sec_l + self.pos.0,ry*sec_l + self.pos.1)
            .color(conrod::color::LIGHT_BLUE)
            .font_size(48)
            .set(self.clock_ids.text_sec, ui);
    }

    fn get_size(&self) -> Vec2u32 {
        self.size
    }
    fn resize(&mut self, size: Vec2u32) {
        self.size = size;
    }

    fn get_position(&self) -> Vec2f64 {
        self.pos
    }
    fn reposition(&mut self, pos: Vec2f64) {
        self.pos = pos;
    }
}
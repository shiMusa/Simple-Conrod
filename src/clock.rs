
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

    frame: Frame<i32>,
    global_center: Vec2<i32>,
}

impl Clock {
    pub fn new(ui: &mut conrod::Ui, time: Arc<RwLock<f64>>, sender: Sender<ClockMsg>) -> Self {
        let clock_ids = ClockIds::new(ui.widget_id_generator());
        Clock { clock_ids, time, sender, frame: Frame{ p0: Vec2{x: 0i32, y: 0i32}, p1: Vec2{x: 100i32, y: 100i32}}, global_center: Vec2{x: 0, y: 0} }
    }
}

impl Element for Clock {
    fn stop(&self) {
        let _ = self.sender.send(ClockMsg::Stop);
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {

        let max = self.frame.min_dim();

        let center = self.frame.center()-self.global_center;

        let sec_r = 0.075 * max as f64;
        let min_r = 0.05 * max as f64;
        let h_r   = 0.045 * max as f64;

        let sec_l = 0.2 * max as f64;
        let min_l = 0.35 * max as f64;
        let h_l   = 0.45 * max as f64;

        let font_size = (0.05 * max as f64) as u32;
        let linewidth = 0.01 * max as f64;


        let t;
        {
            t = *self.time.read().unwrap();
        }

        let hours = (t/(60.0*60.0)) as u8;
        let mins = (t/60.0) as u8;
        let secs = t as u8 - 60*mins;

        let style = widget::primitive::shape::Style::outline_styled(
            widget::primitive::line::Style::solid().thickness(linewidth)
        );

        let mut rx = (t / (60.0 * 60.0 * 60.0) * 2.0 * std::f64::consts::PI).sin();
        let mut ry = (t / (60.0 * 60.0 * 60.0) * 2.0 * std::f64::consts::PI).cos();
        widget::Circle::styled(h_r, style)
            .x_y(rx * h_l + center.x as f64,ry*h_l + center.y as f64)
            .color(conrod::color::LIGHT_BROWN)
            .set(self.clock_ids.circle_h, ui);

        widget::Line::new([center.x as f64, center.y as f64], [rx*(h_l-h_r) + center.x as f64, ry*(h_l-h_r) + center.y as f64])
            .thickness(linewidth)
            .color(conrod::color::LIGHT_BROWN)
            .set(self.clock_ids.line_h, ui);

        widget::Text::new(&format!("{:2}",hours))
            .x_y(rx*h_l + center.x as f64, ry*h_l + center.y as f64)
            .color(conrod::color::LIGHT_BROWN)
            .font_size(font_size)
            .set(self.clock_ids.text_h, ui);


        rx = (t / (60.0 * 60.0) * 2.0 * std::f64::consts::PI).sin();
        ry = (t / (60.0 * 60.0) * 2.0 * std::f64::consts::PI).cos();
        widget::Circle::styled(min_r, style)
            .x_y(rx*min_l + center.x as f64, ry*min_l + center.y as f64)
            .color(conrod::color::LIGHT_GREEN)
            .set(self.clock_ids.circle_min, ui);

        widget::Line::new([center.x as f64, center.y as f64], [rx*(min_l-min_r) + center.x as f64, ry*(min_l-min_r) + center.y as f64])
            .thickness(linewidth)
            .color(conrod::color::LIGHT_GREEN)
            .set(self.clock_ids.line_min, ui);

        widget::Text::new(&format!("{:2}",mins))
            .x_y(rx*min_l + center.x as f64, ry*min_l + center.y as f64)
            .color(conrod::color::LIGHT_GREEN)
            .font_size(font_size)
            .set(self.clock_ids.text_min, ui);


        rx = (t / 60.0 * 2.0 * std::f64::consts::PI).sin();
        ry = (t / 60.0 * 2.0 * std::f64::consts::PI).cos();
        widget::Circle::styled(sec_r, style)
            .x_y(rx*sec_l + center.x as f64, ry*sec_l + center.y as f64)
            .color(conrod::color::LIGHT_BLUE)
            .set(self.clock_ids.circle_sec, ui);

        widget::Line::new([center.x as f64, center.y as f64], [rx*(sec_l-sec_r) + center.x as f64, ry*(sec_l-sec_r) + center.y as f64])
            .thickness(linewidth)
            .color(conrod::color::LIGHT_BLUE)
            .set(self.clock_ids.line_sec, ui);

        widget::Text::new(&format!("{:2}",secs))
            .x_y(rx*sec_l + center.x as f64, ry*sec_l + center.y as f64)
            .color(conrod::color::LIGHT_BLUE)
            .font_size(font_size)
            .set(self.clock_ids.text_sec, ui);
    }

    fn get_frame(&self) -> Frame<i32> { self.frame }
    fn set_frame(&mut self, frame: Frame<i32>) {
        //println!("Clock: setting frame {:?}", frame);
        self.frame = frame;
    }

    fn set_window_center(&mut self, center: Vec2<i32>) {
        self.global_center = center;
    }
}
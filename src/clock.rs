
use elements::*;

use conrod::{self, widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use std::sync::mpsc::{self, Sender, Receiver};
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
                    _ => (),
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
}

impl Clock {
    pub fn new(ui: &mut conrod::Ui, time: Arc<RwLock<f64>>, sender: Sender<ClockMsg>) -> Self {
        let clock_ids = ClockIds::new(ui.widget_id_generator());
        Clock { clock_ids, time, sender }
    }
}

impl Element for Clock {
    fn stop(&self) {
        self.sender.send(ClockMsg::Stop);
    }
    fn build_window(&self, ui: &mut conrod::UiCell) {

        let mut t = 0.0;
        {
            t = *self.time.read().unwrap();
        }

        //println!("{}",t);

        let hours = (t/(60.0*60.0)) as u8;
        let mins = (t/60.0) as u8;
        let secs = t as u8 - 60*mins;

        let style = widget::primitive::shape::Style::outline_styled(
            widget::primitive::line::Style::solid().thickness(5.0)
        );

        let mut rx = (t / (60.0 * 60.0 * 60.0) * 2.0 * std::f64::consts::PI).sin();
        let mut ry = (t / (60.0 * 60.0 * 60.0) * 2.0 * std::f64::consts::PI).cos();
        widget::Circle::styled(40.0, style)
            .x_y(rx*600.0,ry*600.0)
            .color(conrod::color::LIGHT_BROWN)
            .set(self.clock_ids.circle_h, ui);

        widget::Line::new([0.0, 0.0], [rx*(600.0-40.0),ry*(600.0-40.0)])
            .thickness(5.0)
            .color(conrod::color::LIGHT_BROWN)
            .set(self.clock_ids.line_h, ui);

        widget::Text::new(&format!("{:2}",hours))
            .x_y(rx*600.0,ry*600.0)
            .color(conrod::color::LIGHT_BROWN)
            .font_size(48)
            .set(self.clock_ids.text_h, ui);


        rx = (t / (60.0 * 60.0) * 2.0 * std::f64::consts::PI).sin();
        ry = (t / (60.0 * 60.0) * 2.0 * std::f64::consts::PI).cos();
        widget::Circle::styled(66.0, style)
            .x_y(rx*500.0,ry*500.0)
            .color(conrod::color::LIGHT_GREEN)
            .set(self.clock_ids.circle_min, ui);

        widget::Line::new([0.0, 0.0], [rx*(500.0-66.0),ry*(500.0-66.0)])
            .thickness(5.0)
            .color(conrod::color::LIGHT_GREEN)
            .set(self.clock_ids.line_min, ui);

        widget::Text::new(&format!("{:2}",mins))
            .x_y(rx*500.0,ry*500.0)
            .color(conrod::color::LIGHT_GREEN)
            .font_size(48)
            .set(self.clock_ids.text_min, ui);


        rx = (t / 60.0 * 2.0 * std::f64::consts::PI).sin();
        ry = (t / 60.0 * 2.0 * std::f64::consts::PI).cos();
        widget::Circle::styled(100.0, style)
            .x_y(rx*200.0,ry*200.0)
            .color(conrod::color::LIGHT_BLUE)
            .set(self.clock_ids.circle_sec, ui);

        widget::Line::new([0.0, 0.0], [rx*100.0,ry*100.0])
            .thickness(5.0)
            .color(conrod::color::LIGHT_BLUE)
            .set(self.clock_ids.line_sec, ui);

        widget::Text::new(&format!("{:2}",secs))
            .x_y(rx*200.0,ry*200.0)
            .color(conrod::color::LIGHT_BLUE)
            .font_size(48)
            .set(self.clock_ids.text_sec, ui);
    }
}
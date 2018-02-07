
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



            let t = time::precise_time_ns() - t0;
            let mut linkdat = self.time.write().unwrap();
            *linkdat = (t as f64) * 1e-9;



            let dt = time::precise_time_ns() - old;
            //let fdt = (dt as f64) * 1e-9;
            //println!("fps = {:8.0}, dt = {:.6}", 1.0/fdt, fdt);

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
    }
);



pub struct Clock {
    simple_window: BaseWindow,
    clock_ids: ClockIds,

    time: Option<Arc<RwLock<f64>>>,
    sender: Option<Sender<ClockMsg>>,
}

impl Clock {
    pub fn setup(&mut self, time: Arc<RwLock<f64>>, sender: Sender<ClockMsg>) {
        self.time = Some(time);
        self.sender = Some(sender);
    }
}

impl Window for Clock {
    fn new(title: String, width: u32, height: u32) -> Self {
        let mut sw = BaseWindow::new(title, width, height);
        let ids = ClockIds::new(sw.ui.widget_id_generator());
        Clock {
            simple_window: sw,
            clock_ids: ids,
            time: None,
            sender: None,
        }
    }
    fn run(&mut self, fps: f64) {
        self.simple_window.run(fps);
    }
    fn build_window(&mut self) {
        self.simple_window.build_window();

        let sw = &mut self.simple_window;
        let ui = &mut sw.ui.set_widgets();


        let mut t = 0.0;
        {
            let loc_time = self.time.clone().unwrap();
            t = *loc_time.read().unwrap();
        }

        println!("{}",t);

        let hours = (t/(60.0*60.0)) as u8;
        let mins = (t/60.0) as u8;
        let secs = t as u8 - 60*mins;


        let mut rx = (t / (60.0 * 60.0) * 2.0 * std::f64::consts::PI).sin();
        let mut ry = (t / (60.0 * 60.0) * 2.0 * std::f64::consts::PI).cos();
        let style = widget::primitive::shape::Style::outline_styled(
            widget::primitive::line::Style::solid().thickness(5.0)
        );
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
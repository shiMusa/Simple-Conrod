#![feature(duration_extras)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]



extern crate sabi;


use sabi::{elements};
use elements::{*, action::*, basic::*, container::*};
use std::sync::mpsc::{self, Sender, Receiver};





pub struct AnimClick;
impl SizeAnimation for AnimClick {
    fn calc(&self, t: f64, duration: f64) -> (Dim, Dim) {
        let rel = t/duration;
        use std::f64;
        let tau = rel * f64::consts::PI;
        let f = Dim::Relative( 1.0 - 0.25 * tau.sin() );
        (f,f)
    }
}
impl PositionAnimation for AnimClick {
    fn calc(&self, t: f64, duration: f64) -> (Dim, Dim) {
        let rel = t/duration;
        use std::f64;
        let tau = rel * f64::consts::PI * 2.0;
        (
            Dim::Absolute( 0 ),
            Dim::Absolute( (100.0 * tau.sin()) as i32 )
        )
    }
}





fn main() {

    let (sender, receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();

    // setup timer for continuous refresh of window
    let (timer_sender, timer_receiver): (Sender<ActionMsg>, Receiver<ActionMsg>)
        = mpsc::channel();
    let timer = sabi::Timer::new(sender.clone(), timer_receiver, 120.0);

    // construct window

    let pad = Pad::new(Button::new()
        .with_label("Press".to_string())
        .with_id("testbutton".to_string())
        .with_sender(sender.clone()),
        PadAlignment::Center,
        PadElementSize::Positive(Dim::Relative(0.5),Dim::Relative(0.5))
    );


    /** first the long animations, then the short ones.
     * Otherwise, you could get flickering due to reset of
     * inner animations while outer ones still running.
     **/
    let animation = Animation::new(pad)
        .with_duration(1000.0)
        .with_size_animation(Box::new(AnimClick));

    let animation2 = Animation::new(animation)
        .with_duration(250.0)
        .with_position_animation(Box::new(AnimClick));

    let socket = Socket::new(animation2)
        .with_action_receive(Box::new(|ani, amsg|{
            match (amsg.sender_id.as_ref(), amsg.msg) {
                ("testbutton", ActionMsgData::Click) => {
                    ani.start();
                },
                _ => ()
            }
        }));


    let mut window = Window::new("Animation Test".to_string(), 800,800);
    window.add_receiver(receiver);
    window.add_sender(timer_sender);

    window.add_element(socket);
    window.run(120f64);
}

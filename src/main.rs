#![feature(duration_extras)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]



extern crate sabi;


use sabi::{elements};
use elements::{*, action::*, basic::*, container::*};
use std::sync::mpsc::{self, Sender, Receiver};



fn main() {

    let (sender, receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();

    let (timer_sender, timer_receiver): (Sender<ActionMsg>, Receiver<ActionMsg>)
        = mpsc::channel();
    let timer = sabi::Timer::new(sender.clone(), timer_receiver, 120.0);

    let button = Pad::new(Button::new()
        .with_label("Press".to_string())
        .with_id("testbutton".to_string())
        .with_sender(sender.clone()),
        PadAlignment::Center,
        PadElementSize::Positive(Dim::Relative(0.5),Dim::Relative(0.5))
    );


    let animation = Animation::new(1000.0)
        .with_function(AnimationFunction::Size(Box::new(|t|{
            use std::f64;
            let tau = t / 1000.0 * f64::consts::PI * 2.0;
            let f = Dim::Relative( 1.0 + 0.25 * tau.sin() );
            (f,f)
        })));


    let multi_socket = MultiSocket::new(button)
        .with_action_receive(ActionMsg{
            sender_id: "testbutton".to_string(),
            msg: ActionMsgData::Click
        }, Box::new(animation));


    let mut window = BaseWindow::new("Animation Test".to_string(), 800,800);
    window.add_receiver(receiver);
    window.add_sender(timer_sender);
    window.add_element(multi_socket);
    window.run(120f64);
}

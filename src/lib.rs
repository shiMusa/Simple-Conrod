
#![feature(duration_extras)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

pub mod elements;

#[macro_use] extern crate conrod;
extern crate time;
extern crate num;


use elements::{*, container::*, basic::*, action::*};
use std::sync::mpsc::{self, Sender, Receiver};







/*
d888888b d888888b .88b  d88. d88888b d8888b.
`~~88~~'   `88'   88'YbdP`88 88'     88  `8D
   88       88    88  88  88 88ooooo 88oobY'
   88       88    88  88  88 88~~~~~ 88`8b
   88      .88.   88  88  88 88.     88 `88.
   YP    Y888888P YP  YP  YP Y88888P 88   YD


*/



use std::thread;

pub struct Timer {
    handle: thread::JoinHandle<()>,
}
impl Timer {
    pub fn new(sender: Sender<ActionMsg>, receiver: Receiver<ActionMsg>, fps: f64) -> Self {
        use std::thread;
        use std::time::Duration;

        let dur = Duration::from_millis((1000.0/fps) as u64);

        let handle = thread::spawn(move ||{
            'run: loop {
                'receive: loop {
                    match receiver.try_recv() {
                        Ok(msg) => {
                            //println!("Timer: message received: {:?}", msg);

                            match msg.msg {
                                ActionMsgData::Exit => break 'run,
                                _ => ()
                            }

                        },
                        _ => break 'receive
                    }
                }
                let _ = sender.send(ActionMsg{
                    sender_id: "timer".to_string(),
                    msg: ActionMsgData::Update,
                });
                thread::sleep(dur);
            }
        });
        Timer{
            handle
        }
    }

    pub fn stop(self) {
        let _ = self.handle.join();
    }
}











/*
d88888b db    db  .d8b.  .88b  d88. d8888b. db      d88888b
88'     `8b  d8' d8' `8b 88'YbdP`88 88  `8D 88      88'
88ooooo  `8bd8'  88ooo88 88  88  88 88oodD' 88      88ooooo
88~~~~~  .dPYb.  88~~~88 88  88  88 88~~~   88      88~~~~~
88.     .8P  Y8. 88   88 88  88  88 88      88booo. 88.
Y88888P YP    YP YP   YP YP  YP  YP 88      Y88888P Y88888P


*/





pub fn expample2() {


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


    let (sender, receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();

    // setup timer for continuous refresh of window
    let (timer_sender, timer_receiver): (Sender<ActionMsg>, Receiver<ActionMsg>)
        = mpsc::channel();
    let _timer = Timer::new(sender.clone(), timer_receiver, 120.0);

    // construct window

    let pad = Pad::new(Button::new()
        .with_label("Press".to_string())
        .with_id("testbutton".to_string())
        .with_sender(sender.clone()),
        PadAlignment::Center,
        PadElementSize::Positive(Dim::Relative(0.5),Dim::Relative(0.5))
    );


    #[allow(unused_doc_comment)]
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
    window.run();
}





pub fn example() {

    let mut window = Window::new("Container".to_string(), 800, 800);
    let (base_sender, base_receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();
    window.add_receiver(base_receiver);

    let mut layers = Layers::new();

    let mut list = List::new(ListAlignment::Vertical);

    let mut sublist = List::new(ListAlignment::Horizontal);

    sublist.push(
        Button::new()
            // custon action...
            .with_action_click(Box::new(|| {
                println!("List -> List -> Button 1");
            }))
            .with_label("Delete".to_string())
            // we need to define an id if we want to identify the button
            .with_id("Delete".to_string())
            // this sender will send the signals of the button
            // to the main window, which in turn will
            // transmit it down the chain of elements
            .with_sender(base_sender.clone()),
    );
    sublist.push(
        Pad::new(
            Button::new()
                .with_action_click(Box::new(|| {
                    println!("List -> List -> Pad -> Button");
                }))
                .with_label("Hey".to_string())
                .with_id("Hey".to_string())
                .with_sender(base_sender.clone()),
            PadAlignment::Center,
            PadElementSize::Positive(Dim::Relative(0.5), Dim::Relative(0.5))
        ).with_background(Background::Color(conrod::color::LIGHT_ORANGE))
    );
    sublist.push(
        Button::new()
            .with_action_click(Box::new(|| {
                println!("List -> List -> Button 2");
            }))
            .with_label("Add".to_string())
            .with_id("Add".to_string())
            .with_sender(base_sender.clone())
    );
    list.push(sublist);


    list.push(
        Pad::new(
            Button::new()
                .with_action_click(Box::new(||{
                    println!("List -> Button");
                })),
            PadAlignment::Center,
            PadElementSize::Negative(Dim::Absolute(20),Dim::Absolute(20))
        )
    );


    let mut inner_layer = Layers::new();
    inner_layer.push(Pad::new(
        Button::new()
            .with_action_click( Box::new(|| {
                println!("List -> Pad -> Button with const size");
            })),
        PadAlignment::TopLeft,
        PadElementSize::Positive(Dim::Absolute(200), Dim::Absolute(200)) )
        .with_background(Background::Color(conrod::color::LIGHT_BLUE))
    );

    inner_layer.push(
        Socket::new(
            Label::new_with_font_size("Your Ads here!".to_string(), 60)
                .with_color(conrod::color::RED)
        ).with_action_receive(Box::new(|label, msg|{

            println!("Label receives {:?}", msg.clone());

            match (msg.sender_id.as_ref(), msg.msg) {
                ("Action", ActionMsgData::Click) => {
                    label.set_label("Yeäh!!!".to_string());
                },
                _ => ()
            }

        }))
    );

    list.push(inner_layer);


    layers.push(
        Socket::new(list)
            .with_action_receive(Box::new(|list: &mut List, msg: ActionMsg|{
                match (msg.sender_id.as_ref(), msg.msg) {
                    ("Delete", ActionMsgData::Click) => {
                        let _ = list.pop();
                    },
                    ("Add", ActionMsgData::Click) => {
                        list.push(Label::new_with_font_size("one more time!".to_string(), 42)
                            .with_background(Background::Color(conrod::color::LIGHT_YELLOW)))
                    }
                    _ => ()
                }
            }))
    );


    layers.push(Pad::new(
        Button::new()
            .with_label("action".to_string())
            .with_action_click(Box::new(||{
                println!("Äktschöööööön!!!!");
            }))
            .with_color(conrod::color::LIGHT_GREEN)
            .with_id("Action".to_string())
            .with_sender(base_sender),
        PadAlignment::Center,
        PadElementSize::Positive(Dim::Relative(0.5), Dim::Relative(0.4))
    ));


    window.add_element(Pad::new(
        layers,
        PadAlignment::Center,
        PadElementSize::Negative(Dim::Absolute(25),Dim::Absolute(25))
    ));

    window.run();
}
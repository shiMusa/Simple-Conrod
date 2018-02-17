
#![feature(duration_extras)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

pub mod elements;

#[macro_use] extern crate conrod;
extern crate time;
extern crate num;


use elements::{*, container::*, basic::*, action::*};
use std::sync::mpsc::{self, Sender, Receiver};










use std::thread;

pub struct Timer {
    handle: thread::JoinHandle<()>,
}
impl Timer {
    pub fn new(sender: Sender<ActionMsg>, receiver: Receiver<ActionMsg>, fps: f64) -> Self {
        use std::thread;
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
                sender.send(ActionMsg{
                    sender_id: "timer".to_string(),
                    msg: ActionMsgData::Update,
                });
                thread::sleep_ms((1000.0/fps) as u32);
            }
        });
        Timer{
            handle
        }
    }

    pub fn stop(mut self) {
        let _ = self.handle.join();
    }
}
















pub fn example() {

    let mut base_window = BaseWindow::new("Container".to_string(), 800, 800);
    let (base_sender, base_receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();
    base_window.add_receiver(base_receiver);

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


    base_window.add_element(Pad::new(
        layers,
        PadAlignment::Center,
        PadElementSize::Negative(Dim::Absolute(25),Dim::Absolute(25))
    ));

    base_window.run(-1f64);
}


#![feature(duration_extras)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(use_extern_macros)]

pub mod elements;
pub mod composites;

#[macro_use] extern crate conrod;
extern crate time;
extern crate num;
extern crate find_folder;
extern crate image;

use composites::*;
use elements::{*, container::*, basic::*, action::*, shared::*, structures::*};
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
d88888b db    db        ooooo
88'     `8b  d8'       8P~~~~
88ooooo  `8bd8'       dP
88~~~~~  .dPYb.       V8888b.
88.     .8P  Y8.          `8D
Y88888P YP    YP      88oobY'


*/


pub fn example5() {

    let (sender, receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();

    // construct window
    let mut window = Window::new("Scroll Test".to_string(), 800,800);
    let font = Font::new("NotoSans-Regular".to_string(), 42, conrod::color::BLACK);
    window.add_receiver(receiver);

    window.add_image(
        "JapaneseFan".to_string(),
        &assets.join("images/japanese-fan.png")
    );

    let mut layers = Layers::new();

    // * layer 0 ---------------------------------------------------------

    let plane = Plane::new(Graphic::Texture(
        Texture::new("JapaneseFan".to_string())
            .with_mode(TextureMode::FitMax)
    ));
    layers.push(plane);

    // * layer 1 ---------------------------------------------------------


    let mut scroll = Scroll::new(
        ScrollAlignment::Vertical,
        "Scroll".to_string(),
        sender.clone()
    );

    for i in 0..10 {
        let s = format!("Button {}",i);
        let mut pad = Pad::new(
            Socket::new(
                Button::new()
                    .with_font(font.write( s.clone() ))
                    .with_id(s.clone())
                    .with_sender(sender.clone())
            ).with_action_receive(Box::new(move |_,msg|{
                if msg.sender_id == s 
                    && ( msg.msg == ActionMsgData::Press
                        || msg.msg == ActionMsgData::Click ) {
                    println!("{:?}",msg);
                }
            })),
            PadAlignment::Center,
            PadElementSize::Negative(Dim::Absolute(25), Dim::Absolute(25))
        );
        pad.set_min_size(Vec2{x: 400, y: 150});
        scroll.push(pad);
    }

    let pad = Pad::new(
        scroll,
        PadAlignment::Center,
        PadElementSize::Negative(Dim::Relative(0.2), Dim::Relative(0.1))
    );

    layers.push(pad);
    
    window.add_element(layers);
    window.run();
}





/*
d88888b db    db        j88D
88'     `8b  d8'       j8~88
88ooooo  `8bd8'       j8' 88
88~~~~~  .dPYb.       V88888D
88.     .8P  Y8.          88
Y88888P YP    YP          VP


*/



pub fn example4() {

    let (_sender, receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();

    // construct window
    let mut window = Window::new("Animation Test".to_string(), 800,800);
    window.add_receiver(receiver);

    window.add_image(
        "JapaneseFan".to_string(),
        &assets.join("images/japanese-fan.png")
    );

    let mut layers = Layers::new();

    let plane = Plane::new(Graphic::Texture(
        Texture::new("JapaneseFan".to_string())
            .with_mode(TextureMode::FitMax)
    ));

    let pad = Pad::new(
        plane.clone(),
        PadAlignment::Center,
        PadElementSize::Negative(Dim::Absolute(100), Dim::Absolute(100))
    );

    layers.push(pad);

    let pad = Pad::new(
        plane,
        PadAlignment::BottomLeft,
        PadElementSize::Positive(Dim::Absolute(200), Dim::Absolute(200))
    );

    layers.push(pad);



    let padbutton = Pad::new(
        Button::new(),
        PadAlignment::Center,
        PadElementSize::Positive(Dim::Relative(2.0), Dim::Relative(0.1))
    );
    layers.push(padbutton);

    let super_pad = Pad::new(
        layers,
        PadAlignment::TopRight,
        PadElementSize::Positive(Dim::Relative(0.8), Dim::Relative(0.8))
    ).with_crop(true);

    window.add_element(super_pad);
    window.run();
}





/*
d88888b db    db      d8888b.
88'     `8b  d8'      VP  `8D
88ooooo  `8bd8'         oooY'
88~~~~~  .dPYb.         ~~~b.
88.     .8P  Y8.      db   8D
Y88888P YP    YP      Y8888P'


*/





pub fn example3() {

    pub struct AnimClick;
    impl SizeAnimation for AnimClick {
        fn calc(&self, t: f64, duration: f64) -> (Dim, Dim) {
            let rel = t/duration;
            use std::f64;
            let tau = rel * f64::consts::PI;
            let fx = Dim::Relative( -0.25 * tau.sin() );
            let fy = Dim::Relative( 2.0 * tau.sin() );
            (fx,fy)
        }
    }

    let (sender, receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();

    // setup timer for continuous refresh of window
    let (timer_sender, timer_receiver): (Sender<ActionMsg>, Receiver<ActionMsg>)
        = mpsc::channel();
    let _timer = Timer::new(sender.clone(), timer_receiver, 120.0);

    // construct window
    let mut window = Window::new("Animation Test".to_string(), 800,800);
    let font = Font::new("NotoSans-Regular".to_string(), 42, conrod::color::BLACK);
    window.add_receiver(receiver);
    window.add_sender(timer_sender);

    let mut scroll = Scroll::new(
        ScrollAlignment::Vertical,
        "Scroll".to_string(),
        sender.clone()
    );

    for i in 0..10 {
        let s = format!("Button {}", i);

        let button = Button::new()
            .with_font(font.write(s.clone()))
            .with_sender(sender.clone())
            .with_id(s.clone());
        
        let mut pad = Pad::new(
            button,
            PadAlignment::Center,
            PadElementSize::Negative(Dim::Absolute(25),Dim::Absolute(25))
        );
        pad.set_min_size(Vec2{x: 200, y: 100});

        let animation = Animation::new(pad)
            .with_duration(500.0)
            .with_size_animation(Box::new(AnimClick))
            .with_floating(true);

        let socket = Socket::new(animation)
            .with_action_receive(Box::new(move |ani: &mut Animation, amsg: ActionMsg|{   
                if amsg.msg == ActionMsgData::Click && amsg.sender_id == s {
                    println!("{}",s);
                    ani.start();
                }
            }
        ));

        scroll.push(socket);
    }

    window.add_element(scroll);
    window.run();
}













/*
d88888b db    db      .d888b.
88'     `8b  d8'      VP  `8D
88ooooo  `8bd8'          odD'
88~~~~~  .dPYb.        .88'
88.     .8P  Y8.      j88.
Y88888P YP    YP      888888D


*/






pub fn expample2() {


    pub struct AnimClick;
    impl SizeAnimation for AnimClick {
        fn calc(&self, t: f64, duration: f64) -> (Dim, Dim) {
            let rel = t/duration;
            use std::f64;
            let tau = rel * f64::consts::PI;
            let f = Dim::Relative( -0.25 * tau.sin() );
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
                Dim::Absolute( (10.0 * tau.sin()) as i32 )
            )
        }
    }


    let (sender, receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();

    // setup timer for continuous refresh of window
    let (timer_sender, timer_receiver): (Sender<ActionMsg>, Receiver<ActionMsg>)
        = mpsc::channel();
    let _timer = Timer::new(sender.clone(), timer_receiver, 120.0);

    // construct window
    let mut window = Window::new("Animation Test".to_string(), 800,800);
    let font = Font::new("NotoSans-Regular".to_string(), 42, conrod::color::BLACK);
    window.add_receiver(receiver);
    window.add_sender(timer_sender);

    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();

    window.add_font(
        "NotoSans-Italic".to_string(),
        &assets.join("fonts/NotoSans/NotoSans-Italic.ttf")
    );
    let _font_italic = Font::new("NotoSans-Italic".to_string(), 42, conrod::color::BLACK);
    window.add_font(
        "NotoSans-Bold".to_string(),
        &assets.join("fonts/NotoSans/NotoSans-Bold.ttf")
    );
    window.add_font(
        "NotoSans-BoldItalic".to_string(),
        &assets.join("fonts/NotoSans/NotoSans-BoldItalic.ttf")
    );

    window.add_image(
        "RustLogo_hover".to_string(),
        &assets.join("images/rust_hover.png")
    );

    window.add_image(
        "JapaneseFan".to_string(),
        &assets.join("images/japanese-fan.png")
    );

    // add elements to window

    let mut layers = Layers::new();

    layers.push(Plane::new(Graphic::Texture(
        Texture::new("JapaneseFan".to_string())
            .with_mode(TextureMode::FitMax)
    )));



    let pad = Pad::new(Button::new()
        .with_graphic(Graphic::Texture(
            Texture::new("RustLogo_hover".to_string())
            ))
        .with_font(font.write("Press".to_string()))
        .with_id("testbutton".to_string())
        .with_sender(sender.clone()),
        PadAlignment::Center,
        PadElementSize::Positive(Dim::Absolute(200),Dim::Absolute(200))
    );

    #[allow(unused_doc_comment)]
    /** first the long animations, then the short ones.
     * Otherwise, you could get flickering due to reset of
     * inner animations while outer ones still running.
     **/
    let animation = Animation::new(pad)
        .with_duration(500.0)
        .with_size_animation(Box::new(AnimClick));

    let animation2 = Animation::new(animation)
        .with_duration(375.0)
        .with_position_animation(Box::new(AnimClick));

    let socket = Socket::new(animation2)
        .with_action_receive(Box::new(|ani, amsg|{
            match (amsg.sender_id.as_ref(), amsg.msg) {
                ("testbutton", ActionMsgData::Click) => {
                    ani.start();
                },
                _ => ()
            }
        })
    );

    layers.push(socket);

    window.add_element(layers);
    window.run();
}



/*
d88888b db    db
88'     `8b  d8'
88ooooo  `8bd8'
88~~~~~  .dPYb.
88.     .8P  Y8.
Y88888P YP    YP


*/


pub fn example() {

    let mut window = Window::new("Container".to_string(), 800, 800);
    let mut font = Font::new("NotoSans-Regular".to_string(), 42, conrod::color::BLACK);
    let (base_sender, base_receiver): (Sender<ActionMsg>, Receiver<ActionMsg>) = mpsc::channel();
    window.add_receiver(base_receiver);

    let mut layers = Layers::new();

    let mut list = List::new(ListAlignment::Vertical);

    let mut sublist = List::new(ListAlignment::Horizontal);

    sublist.push(
        Button::new()
            .with_font(font.write("Delete".to_string()))
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
                .with_font(font.write("Hey".to_string()))
                .with_id("Hey".to_string())
                .with_sender(base_sender.clone()),
            PadAlignment::Center,
            PadElementSize::Positive(Dim::Relative(0.5), Dim::Relative(0.5))
        )
    );
    sublist.push(
        Button::new()
            .with_font(font.write("Add".to_string()))
            .with_id("Add".to_string())
            .with_sender(base_sender.clone())
    );
    list.push(sublist);


    list.push(
        Pad::new(
            Button::new(),
            PadAlignment::Center,
            PadElementSize::Negative(Dim::Absolute(20),Dim::Absolute(20))
        )
    );


    let mut inner_layer = Layers::new();
    inner_layer.push(Pad::new(
        Button::new(),
        PadAlignment::TopLeft,
        PadElementSize::Positive(Dim::Absolute(200), Dim::Absolute(200)) )
    );

    font.set_size(60);
    let tmp = font.clone();
    inner_layer.push(
        Socket::new(
            Text::new(font.write("Your Ads here!".to_string()))
                .with_color(conrod::color::RED)
        ).with_action_receive(Box::new(move |label, msg|{

            println!("Label receives {:?}", msg.clone());

            match (msg.sender_id.as_ref(), msg.msg) {
                ("Action", ActionMsgData::Click) => {
                    label.set_font(tmp.write("Yeäh!!!".to_string()));
                },
                _ => ()
            }

        }))
    );

    list.push(inner_layer);

    font.set_size(42);
    let tmp = font.clone();
    layers.push(
        Socket::new(list)
            .with_action_receive(Box::new(move |list: &mut List, msg: ActionMsg|{
                match (msg.sender_id.as_ref(), msg.msg) {
                    ("Delete", ActionMsgData::Click) => {
                        let _ = list.pop();
                    },
                    ("Add", ActionMsgData::Click) => {
                        list.push(Text::new(tmp.write("one more time!".to_string())))
                    }
                    _ => ()
                }
            }))
    );


    layers.push(Pad::new(
        Button::new()
            .with_font(font.write("action".to_string()))
            .with_graphic(Graphic::Color(conrod::color::LIGHT_GREEN))
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

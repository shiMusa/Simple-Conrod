

pub mod container;
pub mod basic;
pub mod action;
pub mod shared;
pub mod structures;


use conrod;
use conrod::backend::glium::glium::{self, Surface};
use conrod::position::Rect;
use time;
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::HashMap;
use std::path::Path;

use find_folder;
use image;



const DEBUG: bool = false;



use elements::shared::*;
use elements::action::*;
use elements::structures::*;























/*
                   d888888b d8888b.  .d8b.  d888888b d888888b .d8888.
                   `~~88~~' 88  `8D d8' `8b   `88'   `~~88~~' 88'  YP
                      88    88oobY' 88ooo88    88       88    `8bo.
C8888D C8888D         88    88`8b   88~~~88    88       88      `Y8b.      C8888D C8888D
                      88    88 `88. 88   88   .88.      88    db   8D
                      YP    88   YD YP   YP Y888888P    YP    `8888Y'


*/




use std::i32;

pub trait Element {
    fn setup(&mut self, ui: &mut conrod::Ui);
    fn is_setup(&self) -> bool;

    fn set_parent_widget(&mut self, parent: conrod::widget::id::Id);
    fn set_floating(&mut self, floating: bool);

    fn stop(&mut self) {}
    fn build_window(&self, ui: &mut conrod::UiCell, ressources: &WindowRessources);

    fn get_frame(&self) -> Frame<i32>;
    fn set_frame(&mut self, frame: Frame<i32>, window_center: Vec2<i32>);

    fn set_min_size(&mut self, size: Vec2<i32>);
    fn get_min_size(&self) -> Vec2<i32>;
    fn set_max_size(&mut self, size: Vec2<i32>);
    fn get_max_size(&self) -> Vec2<i32>;

    // * send message. If not processed, send back so that next element can use it.
    fn transmit_msg(&mut self, msg: ActionMsg, stop: bool) -> Option<ActionMsg>;
}

pub trait Labelable {
    fn with_font(self, font: Font) -> Box<Self>;
    fn set_font(&mut self, font: Font);
}

pub trait Colorable {
    fn with_color(self, color: conrod::Color) -> Box<Self>;
    fn set_color(&mut self, color: conrod::Color);
}


pub trait Graphicable {
    fn with_graphic(self, fg: Graphic) -> Box<Self>;
    fn set_graphic(&mut self, fg: Graphic);
}



















/*
db   d8b   db d888888b d8b   db d8888b.  .d88b.  db   d8b   db
88   I8I   88   `88'   888o  88 88  `8D .8P  Y8. 88   I8I   88
88   I8I   88    88    88V8o 88 88   88 88    88 88   I8I   88
Y8   I8I   88    88    88 V8o88 88   88 88    88 Y8   I8I   88
`8b d8'8b d8'   .88.   88  V888 88  .8D `8b  d8' `8b d8'8b d8'
 `8b8' `8d8'  Y888888P VP   V8P Y8888D'  `Y88P'   `8b8' `8d8'


*/

pub struct WindowRessources {
    fonts: HashMap<String, conrod::text::font::Id>,
    image_map: conrod::image::Map<glium::texture::Texture2d>,
    images: HashMap<String, (u32, u32, conrod::image::Id)>
}
impl WindowRessources {
    pub fn new() -> Self {
        WindowRessources {
            fonts: HashMap::new(),
            image_map: conrod::image::Map::new(),
            images: HashMap::new(),
        }
    }

    pub fn add_font(&mut self,  ui: &mut conrod::Ui, id: String, path: &Path) {
        self.fonts.insert(
            id,
            ui.fonts.insert_from_file(path).unwrap()
        );
        println!("font loaded into windowressource.");
        println!("self.fonts {:?}", self.fonts);
    }

    pub fn font(&self, id: &String) -> Option<&conrod::text::font::Id> {
        self.fonts.get(id)
    }

    pub fn image(&self, id: &String) -> Option<&(u32,u32,conrod::image::Id)> {
        self.images.get(id)
    }

    pub fn add_image(&mut self, display: &glium::Display, id: String, path: &Path) {
        let img = Self::load_image(display, path);
        let (w,h) = (img.get_width(), img.get_height().unwrap());
        self.images.insert(
            id,
            (
                w, h,
                self.image_map.insert(
                    Self::load_image(display, path)
                )
            )
        );
        println!("image loaded into windowressource.");
        println!("self.images {:?}", self.images);
    }

    // ? from conrod-example "image_button.rs"
    // Load an image from our assets folder as a texture we can draw to the screen.
    fn load_image<P>(display: &glium::Display, path: P) -> glium::texture::Texture2d
        where P: AsRef<Path>,
    {
        let path = path.as_ref();
        let rgba_image = image::open(&Path::new(&path)).unwrap().to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
        texture
    }
}


widget_ids!(
    struct WindowIds {
        window
    }
);


pub struct Window {
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    renderer: conrod::backend::glium::Renderer,
    //image_map: conrod::image::Map<glium::texture::Texture2d>,
    ui: conrod::Ui,

    element: Option<Box<Element>>,
    receivers: Vec<Receiver<ActionMsg>>,
    senders: Vec<Sender<ActionMsg>>,

    selfsender: Sender<ActionMsg>,

    ids: Option<WindowIds>,
    ressources: WindowRessources,
}

impl Window {

    fn setup(&mut self) {
        let ids = WindowIds::new(self.ui.widget_id_generator());
        println!("setup(): starting...");
        if let Some(ref mut el) = self.element {
            el.set_parent_widget(ids.window);
            el.setup(&mut self.ui);
            println!("setup(): element setup.");
        }
        self.ids = Some(ids);
        println!("setup(): done.");
    }



    pub fn add_font(&mut self, id: String, path: &Path) {
        self.ressources.add_font(
            &mut self.ui,
            id, 
            path
        );
    }

    pub fn add_image(&mut self, id: String, path: &Path) {
        self.ressources.add_image(
            &mut self.display, 
            id, 
            path
        );
    }




    pub fn add_element(&mut self, element: Box<Element>) {
        self.element = Some(element);
    }

    pub fn add_receiver(&mut self, receiver: Receiver<ActionMsg>) {
        self.receivers.push(receiver);
    }

    pub fn add_sender(&mut self, sender: Sender<ActionMsg>) {
        self.senders.push(sender);
    }

    fn send(&mut self, msg: ActionMsgData) {
        for sender in &mut self.senders {
            let tmp = msg.clone();
            let _ = sender.send(ActionMsg{
                sender_id: "Window".to_string(),
                msg: tmp
            });
        }
        
        let _ = self.selfsender.send(ActionMsg{
            sender_id: "Window".to_string(),
            msg
        });
    }


    pub fn new(title: String, width: u32, height: u32) -> Self {
        // build window
        let events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(
            window, context, &events_loop
        ).unwrap();



        // create conrod ui
        let ui =  conrod::UiBuilder::new(
            [width as f64, height as f64]
        ).build();

        // storage for any ressource
        let ressources = WindowRessources::new();

        // connect conrod::render::Primitives to glium Surface
        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        let (selfsender, selfreceiver): (Sender<ActionMsg>, Receiver<ActionMsg>)
            = mpsc::channel();


        Window {
            events_loop,
            display,
            renderer,
            //image_map,
            ui,
            element: None,
            receivers: vec![selfreceiver],
            senders: Vec::new(),
            selfsender,
            ids: None,
            ressources
        }
    }

    pub fn run(&mut self) {
        self.run_with_fps(-1f64);
    }

    pub fn run_with_fps(&mut self, fps: f64) {

        self.setup();

        println!("run(): window setup.");

        let dt_ns = (1.0e9/fps) as u64;

        // events
        let mut events = Vec::new();
        let mut t0 = time::precise_time_ns();

        let mut window_frame = Frame::new();


        // mouse helper
        struct MouseDrag {
            pub current: (f64,f64),
            pub left: bool,
            pub right: bool,
            pub middle: bool,
            start_left: (f64,f64),
            start_right: (f64,f64),
            start_middle: (f64, f64),
        }
        impl MouseDrag {
            pub fn update(&mut self, x: f64, y: f64) {
                self.current = (x,y);
            }

            pub fn start_left(&mut self) {
                self.left = true;
                self.start_left = self.current;
            }
            pub fn start_right(&mut self) {
                self.right = true;
                self.start_right = self.current;
            }
            pub fn start_middle(&mut self) {
                self.middle = true;
                self.start_middle = self.current;
            }

            pub fn stop_left(&mut self) {
                self.left = false;
            }
            pub fn stop_right(&mut self) {
                self.right = false;
            }
            pub fn stop_middle(&mut self) {
                self.middle = false;
            }
            
            pub fn get_left(&self) -> ActionMsgData {
                let (x,y) = self.current;
                let (xx,yy) = (x-self.start_left.0, y-self.start_left.1);
                ActionMsgData::MouseDragLeft(xx,yy)
            }
            pub fn get_right(&self) -> ActionMsgData {
                let (x,y) = self.current;
                let (xx,yy) = (x-self.start_right.0, y-self.start_right.1);
                ActionMsgData::MouseDragRight(xx,yy)
            }
            pub fn get_middle(&self) -> ActionMsgData {
                let (x,y) = self.current;
                let (xx,yy) = (x-self.start_middle.0, y-self.start_middle.1);
                ActionMsgData::MouseDragMiddle(xx,yy)
            }
        }

        let mut mouse_drag = MouseDrag{
            current: (0.0,0.0),
            left: false, right: false, middle: false,
            start_left: (0.0,0.0),
            start_right: (0.0,0.0),
            start_middle: (0.0,0.0)
        };


        let mut window_height = 0;


        //println!("loop is starting ...............................");
        'render: loop {
            events.clear();

            // get new events after last frame
            self.events_loop.poll_events(|event| {
                events.push(event);
            });


            let mut update = false;
            let mut resized = false;

            // process events
            for event in events.drain(..) {

                use conrod::glium::glutin::{Event, WindowEvent, KeyboardInput, VirtualKeyCode,
                MouseButton, ElementState};

                match event.clone() {
                    Event::WindowEvent { event, .. } => {
                        match event {
                            WindowEvent::MouseInput {
                                state,
                                button,
                                ..
                            } => {
                                match state {
                                    ElementState::Pressed => {
                                        let (x,y) = mouse_drag.current;
                                        match button {
                                            MouseButton::Left => {
                                                mouse_drag.start_left();
                                                self.send(ActionMsgData::MousePressLeft(x,y));
                                            },
                                            MouseButton::Right => {
                                                mouse_drag.start_right();
                                                self.send(ActionMsgData::MousePressRight(x,y));
                                            },
                                            MouseButton::Middle => {
                                                mouse_drag.start_middle();
                                                self.send(ActionMsgData::MousePressMiddle(x,y));
                                            },
                                            _ => (),
                                        }
                                    },
                                    ElementState::Released => {
                                        let (x,y) = mouse_drag.current;
                                        match button {
                                            MouseButton::Left => {
                                                mouse_drag.stop_left();
                                                self.send(ActionMsgData::MouseReleaseLeft(x,y));
                                            },
                                            MouseButton::Right => {
                                                mouse_drag.stop_right();
                                                self.send(ActionMsgData::MouseReleaseRight(x,y));
                                            },
                                            MouseButton::Middle => {
                                                mouse_drag.stop_middle();
                                                self.send(ActionMsgData::MouseReleaseMiddle(x,y));
                                            },
                                            _ => (),
                                        }
                                    },
                                }
                            }
                            WindowEvent::CursorMoved {
                                position: (x,y),
                                ..
                            } => {
                                //println!("mouse moved {}, {}",x,y);
                                self.send(ActionMsgData::Mouse(x,window_height as f64 - y));

                                mouse_drag.update(x,window_height as f64 - y);
                                if mouse_drag.left {
                                    self.send(mouse_drag.get_left());
                                }
                                if mouse_drag.right {
                                    self.send(mouse_drag.get_right());
                                }
                                if mouse_drag.middle {
                                    self.send(mouse_drag.get_middle());
                                }
                            },
                            WindowEvent::Closed |
                            WindowEvent::KeyboardInput {
                                input: KeyboardInput{
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                            } => {
                                self.send(ActionMsgData::Exit);
                                use std::thread;
                                use std::time::Duration;
                                thread::sleep(Duration::from_millis(1000));
                                break 'render
                            },
                            WindowEvent::Resized(mut w, mut h) => {

                                window_height = h;

                                // TODO is this limit necessary?
                                if let Some(ref mut el) = self.element {
                                    let min = el.get_min_size();
                                    let max = el.get_max_size();

                                    if w > max.x as u32 { w = max.x as u32; }
                                    if w < min.x as u32 { w = min.x as u32; }
                                    if h > max.y as u32 { h = max.y as u32; }
                                    if h < min.y as u32 { h = min.y as u32; }

                                    window_frame = Frame::new_with_size(w as i32,h as i32);
                                    resized = true;
                                }
                            }
                            _ => (),
                        }
                    },
                    _ => (),
                };

                // convert winit event to conrod input
                let input = match conrod::backend::winit::convert_event(event, &self.display) {
                    None => continue,
                    Some(input) => input,
                };

                // handle input
                self.ui.handle_event(input);

                update = true;
            }

            // check for fps forced update
            if fps > 0.0 {
                let time_diff = time::precise_time_ns() - t0;
                if time_diff >= dt_ns {
                    self.ui.needs_redraw();
                    t0 = time::precise_time_ns();
                    update = true;
                }
            }

            // check if msgs have to be processed and transmit through chain
            for receiver in &self.receivers {
                'receive: loop {
                    match receiver.try_recv() {
                        Ok(msg) => {
                            if DEBUG { println!("message received: {:?}", msg); }
                            if let Some(ref mut el) = self.element {
                                el.transmit_msg(msg.clone(), false);
                            }
                            update = true;
                            for sender in &mut self.senders {
                                let _ = sender.send(msg.clone());
                            }

                            match msg.msg {
                                ActionMsgData::Update => self.ui.needs_redraw(),
                                _ => ()
                            }
                        },
                        _ => break 'receive
                    }
                }
            }

            if let Some(ref mut el) = self.element {
                if !el.is_setup() {
                    el.setup(&mut self.ui);
                    update = true;
                }
            }

            if resized {
                if let Some(ref mut el) = self.element {
                    el.set_frame(window_frame, window_frame.center());
                }
            }

            if update {
                let ui = &mut self.ui.set_widgets();
                let res = &self.ressources;
                if DEBUG { println!("run() start building...");}

                if let Some(ref ids) = self.ids {
                    use conrod::{widget, Widget, Colorable};
                    widget::canvas::Canvas::new()
                        .rgba(0.0,0.0,0.0,0.0)
                        .set(ids.window, ui);
                }

                if let Some(ref mut el) = self.element {
                    //el.set_frame(window_frame, window_frame.center());
                    el.build_window(ui, res);
                    if DEBUG { println!("run() element build...");}
                }
                if DEBUG { println!("run() all elements build.");}
            }

            // draw ui if changed
            if let Some(primitives) = self.ui.draw_if_changed() {
                if DEBUG { println!("run() preparing target for drawing...");}
                self.renderer.fill(&self.display, primitives, &self.ressources.image_map);
                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);

                if DEBUG { println!("run() drawing..."); }
                self.renderer.draw(&self.display, &mut target, &self.ressources.image_map).unwrap();
                target.finish().unwrap();
                if DEBUG { println!("run() loop finished."); }
            }
        }

        if let Some(ref mut el) = self.element {
            el.stop();
        }
    }
}





use conrod::{self, widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use conrod::backend::glium::glium::{self, Surface};
use time;









pub trait Window {
    fn new(title: String, width: u32, height: u32) -> Self;
    fn run(&mut self, fps: f64, build_window: &FnMut(Self));
}




pub struct BaseWindow {
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    renderer: conrod::backend::glium::Renderer,
    image_map: conrod::image::Map<glium::texture::Texture2d>,

    pub ui: conrod::Ui,
}

impl BaseWindow {
    fn build_window(&mut self) { }
}

impl Window for BaseWindow {
    fn new(title: String, width: u32, height: u32) -> Self {
        // build window
        let events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(8);
        let display = glium::Display::new(
            window, context, &events_loop
        ).unwrap();



        // create conrod ui
        let mut ui =  conrod::UiBuilder::new(
            [width as f64, height as f64]
        ).build();


        // Add a font to the ui's font::map from file
        const FONT_PATH: &'static str =
            concat!(env!("CARGO_MANIFEST_DIR"),
                "\\assets\\fonts\\NotoSans\\NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(FONT_PATH).unwrap();

        // connect conrod::render::Primitives to glium Surface
        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // image mapping, here: none
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();


        BaseWindow {
            events_loop,
            display,
            renderer,
            image_map,
            ui
        }
    }

    fn run(&mut self, fps: f64, build_window: &FnMut(Self)) {
        let dt_ns = (1.0e9/fps) as u64;

        // events
        let mut events = Vec::new();
        let mut t0 = time::precise_time_ns();
        'render: loop {
            events.clear();

            // get new events after last frame
            self.events_loop.poll_events(|event| {
                events.push(event);
            });


            let mut update = false;

            // process events
            for event in events.drain(..) {

                use conrod::glium::glutin::{Event, WindowEvent, KeyboardInput, VirtualKeyCode};

                match event.clone() {
                    Event::WindowEvent { event, .. } => {
                        match event {
                            WindowEvent::Closed |
                            WindowEvent::KeyboardInput {
                                input: KeyboardInput{
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                            } => break 'render,
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

            let time_diff = time::precise_time_ns() - t0;
            if time_diff >= dt_ns {
                self.ui.needs_redraw();
                t0 = time::precise_time_ns();
                update = true;
            }

            if update {
                // add some stuff
                build_window();
            }

            // draw ui if changed
            if let Some(primitives) = self.ui.draw_if_changed() {
                self.renderer.fill(&self.display, primitives, &self.image_map);
                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
}
#![allow(dead_code)]

use glfw::Context;
use std::sync::mpsc::Receiver;

use super::{keyboard::Keyboard, mouse::Mouse};

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    keyboard: Keyboard,
    mouse: Mouse,
}

pub struct WindowArgs {
    pub width: u32,
    pub height: u32,
    pub title: &'static str,
    pub borderless: bool,
}

pub enum WindowError {
    FailedGLFWInit,
    FailedGLFWCreateWindow,
}

pub trait EventReciever {
    fn init(&mut self, window: &mut Window);
    fn render_loop(&mut self, window: &mut Window, dt: f32);
    fn closing(&mut self, window: &mut Window);
}

impl Window {
    pub fn new(args: &WindowArgs) -> Result<Window, WindowError> {
        // Initializing GLFW
        let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
            Err(_) => return Err(WindowError::FailedGLFWInit),
            Ok(glfw) => glfw,
        };

        let mut width = args.width;
        let mut height = args.height;

        if args.borderless {
            glfw.window_hint(glfw::WindowHint::Decorated(false));
            glfw.with_primary_monitor(|_, monitor| {
                monitor.and_then(|monitor| {
                    monitor.get_video_mode().and_then(|video_mode| {
                        width = video_mode.width;
                        height = video_mode.height;
                        Some(video_mode)
                    })
                })
            });
        }

        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::Visible(false));

        let (mut window, events) =
            match glfw.create_window(width, height, args.title, glfw::WindowMode::Windowed) {
                None => return Err(WindowError::FailedGLFWCreateWindow),
                Some((window, events)) => (window, events),
            };

        window.make_current();
        gl::load_with(|s| window.get_proc_address(s));

        let keyboard = Keyboard::new(&mut window);
        let mouse = Mouse::new(&mut window);

        return Ok(Window {
            glfw,
            window,
            events,
            keyboard,
            mouse,
        });
    }

    pub fn show(&mut self, reciever: &mut dyn EventReciever) {
        self.window.show();
        reciever.init(self);

        let mut frame_time = 0.0;

        while !self.window.should_close() {
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                self.keyboard.process_event(&event);
                self.mouse.process_event(&event);
            }

            let time = self.glfw.get_time();
            let delta_time = (time - frame_time) as f32;
            frame_time = time;

            reciever.render_loop(self, delta_time);

            self.keyboard.update();
            self.mouse.update();

            self.window.swap_buffers();
        }

        reciever.closing(self);
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }
}

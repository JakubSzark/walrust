mod context;
mod graphics;
mod math;

use context::keyboard::Key;
use context::window::{EventReciever, Window, WindowArgs, WindowError};
use std::ffi::CStr;

struct Application {
    fps_timer: f32,
}

impl EventReciever for Application {
    fn init(&mut self, _: &mut Window) {
        println!("[App]: Initialized");

        // Prints the current OpenGL version
        unsafe {
            let version_str = gl::GetString(gl::VERSION);
            let version = CStr::from_ptr(version_str as *const i8);
            println!("[App]: OpenGL {}", version.to_string_lossy());
        }
    }

    fn render_loop(&mut self, window: &mut Window, dt: f32) {
        self.fps_timer += dt;
        if self.fps_timer >= 1.0 {
            println!("[App]: {} FPS", (1.0 / dt).floor());
            self.fps_timer = 0.0;
        }

        let keyboard = window.keyboard();
        if keyboard.get_key_down(Key::Escape) {
            println!("[App]: Escape pressed!");
        }
    }

    fn closing(&mut self, _: &mut Window) {
        println!("[App]: Cleaning up");
    }
}

fn main() {
    let args = WindowArgs {
        width: 1280,
        height: 720,
        title: "Walrust",
        borderless: false,
    };

    match Window::new(&args) {
        Err(e) => match e {
            WindowError::FailedGLFWInit => {
                panic!("Failed to initialize glfw!");
            }
            WindowError::FailedGLFWCreateWindow => {
                panic!("Failed to create window!");
            }
        },
        Ok(mut window) => {
            let mut app = Application { fps_timer: 0.0 };
            window.show(&mut app);
        }
    }
}

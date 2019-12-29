
/** REQUIRED FOR WINDOWS BUILD START*/
extern crate shell32;
/** REQUIRED FOR WINDOWS BUILD END*/

extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;

use std::sync::mpsc::Receiver;

pub struct RevColor
{
    r : f32,
    g : f32,
    b : f32,
    a : f32,
}

impl RevColor
{
    fn new(r:f32, g:f32, b:f32, a:f32) -> RevColor
    {
        RevColor{r:r, g:g, b:b, a:a}
    }
}

enum RevColorTypes
{
    Red(RevColor),
    Blue(RevColor),
}

// settings
pub struct StartupSettings {
    WindowWidth: u32,
    WindowHeight: u32,
    title : String,

}

fn create_startup_settings() -> StartupSettings {
    let f = StartupSettings{
        WindowWidth: 1024,
        WindowHeight: 720,
        title: String::from("RevEngine"),
    };
    f
}

fn REV_RED_COLOR() -> RevColor {
    RevColor::new(1.0, 0.0, 0.0, 1.0)
}

fn REV_WHITE_COLOR() -> RevColor {
    RevColor::new(1.0, 1.0, 1.0, 1.0)
}

fn REV_BLACK_COLOR() -> RevColor {
    RevColor::new(0.0, 0.0, 0.0, 1.0)
}

fn clear_color_gl(color:RevColor)
{
    unsafe
    {
        gl::ClearColor(color.r, color.g, color.b, color.a);
    }
}
fn main() 
{
    let startup_settings = create_startup_settings();
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "windows")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(
        startup_settings.WindowWidth,
        startup_settings.WindowHeight, 
        startup_settings.title.as_str(), 
        glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

 
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    while !window.should_close() {

        process_events(&mut window, &events);
        unsafe {
            clear_color_gl(REV_BLACK_COLOR());
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers();
        glfw.poll_events();
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) 
{
    for (_, event) in glfw::flush_messages(events) 
    {
        match event 
        {
            glfw::WindowEvent::FramebufferSize(width, height) => 
            {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

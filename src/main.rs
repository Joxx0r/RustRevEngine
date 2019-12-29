
/** REQUIRED FOR WINDOWS BUILD START*/
extern crate shell32;
/** REQUIRED FOR WINDOWS BUILD END*/

extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;

use std::sync::mpsc::Receiver;

use std::fs::File;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() 
{
    let startup_settings_path = "resources/startup_settings.txt";
    let startup_settings = File::open(startup_settings_path).unwrap_or_else(|error|
    {
        panic!("Failed finding basic startup_settings. Expected path {} Error {:?}", startup_settings_path, error.kind());
    });

    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "RevEngine", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    while !window.should_close() 
    {
        process_events(&mut window, &events);

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

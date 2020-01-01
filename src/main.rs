/** REQUIRED FOR WINDOWS BUILD START*/
extern crate shell32;
/** REQUIRED FOR WINDOWS BUILD END*/

/** OPENGL */
/** START */
extern crate glfw;
use self::glfw::{Context, Key, Action};
extern crate gl;

/** END */
/** OPENGL*/

/** STD */
/** START */
use std::sync::mpsc::Receiver;
use std::ptr;

/** END */
/** STD*/

/** INTERNAL */

/** START MODULES */
mod types;
mod utils;
mod rc;
mod math;
mod model;
mod shader;
mod rc_internal;
/** END MODULES*/

/** START TYPES USE*/
use model::RevModel;
use crate::types::RevColor;
/** END TYPES USE*/
/** INTERNAL*/

fn main() 
{
    let startup_settings = types::create_startup_settings(1024, 720);
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "windows")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(
        startup_settings.window_width,
        startup_settings.window_height, 
        startup_settings.title.as_str(), 
        glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
 
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let model: RevModel = unsafe {

        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);
        // build and compile shaders
        RevModel::new()
        //RevModel::new_from_path(utils::modify_to_resource_path("models/nanosuit/nanosuit.obj").as_str())
    };

     while !window.should_close() {

        process_events(&mut window, &events);

        unsafe {
            rc::clear_color_gl(RevColor::black());
          
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // bind textures on corresponding texture units
            model.Draw();
    
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}


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

/** REQUIRED FOR WINDOWS BUILD START*/
extern crate shell32;
/** REQUIRED FOR WINDOWS BUILD END*/

/** OPENGL */
/** START */
extern crate glfw;
use self::glfw::{Context, Key, Action};
extern crate gl;

use cgmath::{Matrix4,   Deg, perspective, Point3};

/** END */
/** OPENGL*/

/** STD */
/** START */
use std::sync::mpsc::Receiver;
use std::ffi::{CStr};

/** END */
/** STD*/

/** INTERNAL */

/** START MODULES */
mod utils;
mod math;
mod render;
mod core;
mod misc;

/** END MODULES*/

/** START TYPES USE*/
use crate::types::RevColor;
use crate::misc::camera::Camera_Movement::*;
use crate::core::types;
use render::rc;
use crate::render::model::RevModel;
use crate::misc::camera::Camera;
use glfw::MouseButton;
use misc::input::input_state;
use crate::math::vec::Vec2;

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

    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 50.0),
        ..Camera::default()
    };


    let model: RevModel = unsafe {

        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);
        // build and compile shaders
        RevModel::new_from_path(utils::modify_to_resource_path("models/nanosuit/").as_str(), "nanosuit.obj")
    };


   // model.save_to_file(utils::modify_to_resource_path("models/nanosuit/").as_str(), "nanosuit");

    let mut delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;


     while !window.should_close() {

        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        process_events(&mut window, &events);

         let current_frame_input_state : input_state = misc::input::calculate_input(&mut window);
         unsafe
         {
             misc::input::process_input(&mut window, delta_time, &mut camera, current_frame_input_state);
         }
        unsafe {
            rc::clear_color_gl(RevColor::black());
            
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            model.m_shader.use_program();

            // pass projection matrix to shader (note that in this case it could change every frame)
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), startup_settings.window_width as f32 / startup_settings.window_height as f32 , 0.1, 100.0);
            model.m_shader.set_mat_4(c_str!("projection"), &projection);

            // camera/view transformation
            let view = camera.GetViewMatrix();
            model.m_shader.set_mat_4(c_str!("view"), &view);

            // bind textures on corresponding texture units
            model.draw();
    
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

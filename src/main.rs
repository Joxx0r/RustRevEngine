/** REQUIRED FOR WINDOWS BUILD START*/
extern crate shell32;
/** REQUIRED FOR WINDOWS BUILD END*/

/** OPENGL */
/** START */
extern crate glfw;
use self::glfw::{Context, Key, Action};
extern crate gl;

use cgmath::{Matrix4, Vector3, vec3,  Deg, perspective, Point3};

/** END */
/** OPENGL*/

/** STD */
/** START */
use std::sync::mpsc::Receiver;
use std::ptr;
use std::ffi::{CStr, CString};

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
mod camera;
/** END MODULES*/

/** START TYPES USE*/
use model::RevModel;
use crate::types::RevColor;
use crate::camera::Camera;
use crate::camera::Camera_Movement::*;
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
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };


    let model: RevModel = unsafe {

        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);
        // build and compile shaders
        RevModel::new_from_path(utils::modify_to_resource_path("models/nanosuit/").as_str(), "nanosuit.obj")
    };

    let mut deltaTime: f32; // time between current frame and last frame
    let mut lastFrame: f32 = 0.0;
    
     while !window.should_close() {

        let currentFrame = glfw.get_time() as f32;
        deltaTime = currentFrame - lastFrame;
        lastFrame = currentFrame;

        process_events(&mut window, &events);

        processInput(&mut window, deltaTime, &mut camera);

        
        unsafe {
            rc::clear_color_gl(RevColor::black());
            
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            model.m_shader.useProgram();

            // pass projection matrix to shader (note that in this case it could change every frame)
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), startup_settings.window_width as f32 / startup_settings.window_height as f32 , 0.1, 100.0);
            model.m_shader.setMat4(c_str!("projection"), &projection);

            // camera/view transformation
            let view = camera.GetViewMatrix();
            model.m_shader.setMat4(c_str!("view"), &view);

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

pub fn processInput(window: &mut glfw::Window, deltaTime: f32, camera: &mut Camera) {
    
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    if window.get_key(Key::W) == Action::Press {
        camera.ProcessKeyboard(FORWARD, deltaTime);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.ProcessKeyboard(BACKWARD, deltaTime);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.ProcessKeyboard(LEFT, deltaTime);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.ProcessKeyboard( RIGHT, deltaTime);
    }
}


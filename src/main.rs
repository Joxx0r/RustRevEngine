/** REQUIRED FOR WINDOWS BUILD START*/
extern crate shell32;
/** REQUIRED FOR WINDOWS BUILD END*/

/** OPENGL */
/** START */
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;
/** END */
/** OPENGL*/

/** STD */
/** START */
use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
/** END */
/** STD*/

/** INTERNAL */

/** START MODULES */
mod types;
mod utils;
mod rc;
/** END MODULES*/

/** START TYPES USE*/
use crate::types::RevColor;
mod shader;
use crate::shader::Shader;
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

    let (our_shader, vao) = unsafe {
        let our_shader = Shader::new(
            "../../resources/shaders/shader.vs",
            "../../resources/shaders/shader.fs",
        ).unwrap_or_else(|error|{
            panic!("Failed serializing shader {}", error);
        }); 
        

        let vertices: [f32; 18] = [
            0.5, -0.5, 0.0,  1.0, 0.0, 0.0, 
           -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  
            0.0,  0.5, 0.0,  0.0, 0.0, 1.0   
        ];
        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
       
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;


        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);


        (our_shader, vao)

    };

     while !window.should_close() {

        process_events(&mut window, &events);

   
        unsafe {
            rc::clear_color_gl(RevColor::black());
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // render the triangle
            our_shader.useProgram();
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
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

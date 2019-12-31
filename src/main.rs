/** REQUIRED FOR WINDOWS BUILD START*/
extern crate shell32;
/** REQUIRED FOR WINDOWS BUILD END*/

/** OPENGL */
/** START */
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use image;
use image::GenericImage;

/** END */
/** OPENGL*/

/** STD */
/** START */
use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::{CString, CStr};

use std::mem::size_of;
/** END */
/** STD*/

/** INTERNAL */

/** START MODULES */
mod types;
mod utils;
mod rc;
mod math;
use crate::math::*;
/** END MODULES*/

/** START TYPES USE*/
use crate::types::RevColor;
mod shader;
use crate::shader::Shader;
/** END TYPES USE*/
/** INTERNAL*/

struct Vertex
{
    p : math::Vec3,
    c : math::Vec3,
    t : math::Vec2,
}

impl Vertex 
{
    pub fn new(in_p: math::Vec3,in_c: math::Vec3, in_t: math::Vec2 ) -> Vertex
    {
        Vertex{p:in_p, c:in_c, t:in_t}
    }
}

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

    let (our_shader, vbo, vao, ebo, texture_1, texture_2) = unsafe {

        let our_shader = Shader::new(
            utils::read_resource_path("shaders/shader.vs").as_str(),
            utils::read_resource_path("shaders/shader.fs").as_str(),
        ).unwrap_or_else(|error|{
            panic!("Failed serializing shader: {}", error);
        }); 

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [Vertex; 4] = [
            // positions       // colors        // texture coords
             Vertex::new(Vec3::new_3(0.5,  0.5, 0.0),   Vec3::new_3(1.0, 0.0, 0.0),   Vec2::new_2(1.0, 1.0)), // top right
             Vertex::new(Vec3::new_3(0.5, -0.5, 0.0),   Vec3::new_3( 0.0, 1.0, 0.0),   Vec2::new_2(1.0, 0.0)), // top right
             Vertex::new(Vec3::new_3(-0.5, -0.5, 0.0),   Vec3::new_3(0.0, 0.0, 1.0),   Vec2::new_2(0.0, 0.0)), // top right
             Vertex::new(Vec3::new_3(-0.5,  0.5, 0.0),   Vec3::new_3(1.0, 1.0, 0.0),   Vec2::new_2(0.0, 1.0)), // top right
        ];

        let indices: [u32;6] = [
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let size = (vertices.len() * size_of::<Vertex>()) as isize;
        let data = &vertices[0] as *const Vertex as *const c_void;
        
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        let size = (indices.len() * size_of::<u32>()) as isize;
        let data = &indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        let size = size_of::<Vertex>() as i32;
        // vertex Positions
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, p) as *const c_void);
        // vertex normals
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, c) as *const c_void);
        // vertex texture coords
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, t) as *const c_void);
               
        gl::BindVertexArray(0);

        // load and create a texture
        // -------------------------
        let (mut texture_1, mut texture_2) = (0, 0);
        // texture 1
        // ---------
        gl::GenTextures(1, &mut texture_1);
        gl::BindTexture(gl::TEXTURE_2D, texture_1);
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new(utils::read_resource_path("textures/container.jpg").as_str())).expect("Failed to load texture");
        let data = img.raw_pixels();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGB as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0,
                       gl::RGB,
                       gl::UNSIGNED_BYTE,
                       &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);
        // texture 2
        // ---------
        gl::GenTextures(1, &mut texture_2);
        gl::BindTexture(gl::TEXTURE_2D, texture_2);
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new(utils::read_resource_path("textures/awesomeface.png").as_str())).expect("Failed to load texture");
        let img = img.flipv(); // flip loaded texture on the y-axis.
        let data = img.raw_pixels();
        // note that the awesomeface.png has transparency and thus an alpha channel, so make sure to tell OpenGL the data type is of GL_RGBA
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGB as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0,
                       gl::RGBA,
                       gl::UNSIGNED_BYTE,
                       &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
        // -------------------------------------------------------------------------------------------
        our_shader.useProgram(); // don't forget to activate/use the shader before setting uniforms!
        // either set it manually like so:
        gl::Uniform1i(gl::GetUniformLocation(our_shader.ID, c_str!("texture_1").as_ptr()), 0); // using c_str! macro to avoid runtime overhead
         // or set it via the texture class
        our_shader.setInt(c_str!("texture_2"), 1);

        (our_shader, vbo, vao, ebo, texture_1, texture_2)

    };

     while !window.should_close() {

        process_events(&mut window, &events);

        unsafe {
            rc::clear_color_gl(RevColor::black());
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // bind textures on corresponding texture units
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture_2);


            // render the triangle
            our_shader.useProgram();
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
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

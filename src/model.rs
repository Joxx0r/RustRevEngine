
#![allow(dead_code)]

extern crate glfw;
extern crate gl;

use crate::types::*;
use crate::shader::Shader;
use crate::utils;
use crate::math::*;

use image;
use image::GenericImage;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::{CStr};
use std::mem::size_of;


pub struct Model {
    pub m_shader:Shader,
    pub m_vbo:u32, 
    pub m_vao:u32, 
    pub m_ebo:u32, 
    pub m_texture_1:u32, 
    pub m_texture_2:u32,
}

impl Model
{
    pub fn new() -> Model
    {
        let (our_shader, vbo, vao, ebo, texture_1, texture_2) = unsafe {

        let our_shader = Shader::new(
            utils::read_resource_path("shaders/shader.vs").as_str(),
            utils::read_resource_path("shaders/shader.fs").as_str(),
        ).unwrap_or_else(|error|{
            panic!("Failed serializing shader: {}", error);
        }); 


        let vertices: [VertexPosColTex; 4] = [
            // positions       // colors        // texture coords
            VertexPosColTex::new(Vec3::new_3(0.5,  0.5, 0.0),   Vec3::new_3(1.0, 0.0, 0.0),   Vec2::new_2(1.0, 1.0)), // top right
            VertexPosColTex::new(Vec3::new_3(0.5, -0.5, 0.0),   Vec3::new_3( 0.0, 1.0, 0.0),   Vec2::new_2(1.0, 0.0)), // top right
            VertexPosColTex::new(Vec3::new_3(-0.5, -0.5, 0.0),   Vec3::new_3(0.0, 0.0, 1.0),   Vec2::new_2(0.0, 0.0)), // top right
            VertexPosColTex::new(Vec3::new_3(-0.5,  0.5, 0.0),   Vec3::new_3(1.0, 1.0, 0.0),   Vec2::new_2(0.0, 1.0)), // top right
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

        let size = (vertices.len() * size_of::<VertexPosColTex>()) as isize;
        let data = &vertices[0] as *const VertexPosColTex as *const c_void;

        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        let size = (indices.len() * size_of::<u32>()) as isize;
        let data = &indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        let size = size_of::<VertexPosColTex>() as i32;
        // Vertex Positions
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, offset_of!(VertexPosColTex, location) as *const c_void);
        // Vertex normals
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, size, offset_of!(VertexPosColTex, color) as *const c_void);
        // Vertex texture coords
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, size, offset_of!(VertexPosColTex, tex_coord) as *const c_void);
               
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

        Model {m_shader:our_shader, m_vbo:vbo, m_vao:vao, m_ebo:ebo, m_texture_1:texture_1, m_texture_2:texture_2}
    }
}
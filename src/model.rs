
#![allow(dead_code)]

extern crate glfw;
extern crate gl;

use crate::types::*;
use crate::shader::Shader;
use crate::utils;
use crate::math::*;
use crate::rc_internal;

use image;
use image::GenericImage;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::{CStr, CString};
use std::mem::size_of;

pub struct RevModel {
    pub m_meshes: Vec<RevMesh>,
    pub m_textures: Vec<RevTexture>,   // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    pub m_shader : Shader,
}

pub struct RevMesh {
    pub m_verts: Vec<VertexPosNormTexTanBi>,
    pub m_indicies: Vec<u32>,
    pub m_textures: Vec<RevTexture>,
    pub m_vbo:u32, 
    pub m_vao:u32, 
    pub m_ebo:u32, 
    pub m_texture_1:u32, 
    pub m_texture_2:u32
}

impl RevModel
{
    pub fn default() -> Self
    {
        Self {
            m_meshes : Vec::new(),
            m_textures : Vec::new(),
            m_shader : Shader::default(),
        }
    }

    pub fn new_from_path(path:&str) -> Self
    {   
         // retrieve the directory path of the filepath
         let mut model_instance = RevModel::default();
         
         let path = Path::new(path);
         let obj = tobj::load_obj(path);
 
         let (models, materials) = obj.unwrap();
         for model in models {
             let mesh = &model.mesh;
             let num_vertices = mesh.positions.len() / 3;
 
             // data to fill
             let mut vertices: Vec<VertexPosNormTexTanBi> = Vec::with_capacity(num_vertices);
             let indices: Vec<u32> = mesh.indices.clone();
 
             let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
             for i in 0..num_vertices {
                 vertices.push(
                    VertexPosNormTexTanBi::new(
                        Vec3::new_3(p[i*3], p[i*3+1], p[i*3+2]),  
                        Vec3::new_3(n[i*3], n[i*3+1], n[i*3+2]), 
                        Vec2::new_2(t[i*2], t[i*2+1]), Vec3::zero(), Vec3::zero())); 
             }
 
             // process material
             let mut textures = Vec::new();
             if let Some(material_id) = mesh.material_id {
                 let material = &materials[material_id];
 
                 // 1. diffuse map
                 if !material.diffuse_texture.is_empty() {
                     let texture = rc_internal::load_material_texture(&mut model_instance.m_textures, material.diffuse_texture.as_str(), "texture_diffuse");
                     textures.push(texture);
                 }
                 // 2. specular map
                 if !material.specular_texture.is_empty() {
                     let texture = rc_internal::load_material_texture(&mut model_instance.m_textures, material.specular_texture.as_str(), "texture_specular");
                     textures.push(texture);
                 }
                 // 3. normal map
                 if !material.normal_texture.is_empty() {
                     let texture = rc_internal::load_material_texture(&mut model_instance.m_textures, material.normal_texture.as_str(), "texture_normal");
                     textures.push(texture);
                 }
                 // NOTE: no height maps
             }
             model_instance.m_meshes.push(RevMesh::new(vertices, indices, textures));
         }

         model_instance
    }

    pub fn Draw(&self) {
        unsafe { self.m_shader.useProgram(); }
        for mesh in &self.m_meshes {
            unsafe { mesh.Draw(&self.m_shader); }
        }
    }


    pub fn new() -> RevModel
    {
        RevModel::default()
     /*   let (our_shader, vbo, vao, ebo, texture_1, texture_2) = unsafe {

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

        Model {m_shader:our_shader, m_vbo:vbo, m_vao:vao, m_ebo:ebo, m_texture_1:texture_1, m_texture_2:texture_2}*/
    }

  
}

impl RevMesh
{
    pub fn default() -> Self
    {
        Self 
        {
            m_verts : Vec::new(),
            m_indicies : Vec::new(),
            m_textures : Vec::new(),
            m_vbo : 0,
            m_vao : 0,
            m_ebo : 0,
            m_texture_1 : 0,
            m_texture_2 : 0,
        }
    }

    pub fn new(verts:Vec<VertexPosNormTexTanBi>, indicies:Vec<u32>, textures:Vec<RevTexture>) -> Self
    {
        let mut m = Self
        {
            m_verts : verts,
            m_indicies : indicies,
            m_textures : textures,
            ..RevMesh::default()
        };

        m.SetupMesh();
        m
    }

    fn SetupMesh(&mut self)
    {
        unsafe
        {
            gl::GenVertexArrays(1, &mut self.m_vao);
            gl::GenBuffers(1, &mut self.m_vbo);
            gl::GenBuffers(1, &mut self.m_ebo);
    
            gl::BindVertexArray(self.m_vao);
            // load data into vertex buffers
            gl::BindBuffer(gl::ARRAY_BUFFER, self.m_vbo);
            // A great thing about structs with repr(C) is that their memory layout is sequential for all its items.
            // The effect is that we can simply pass a pointer to the struct and it translates perfectly to a glm::vec3/2 array which
            // again translates to 3/2 floats which translates to a byte array.
            let size = (self.m_verts.len() * size_of::<VertexPosNormTexTanBi>()) as isize;
            let data = &self.m_verts[0] as *const VertexPosNormTexTanBi as *const c_void;
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);
    
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.m_ebo);
            let size = (self.m_indicies.len() * size_of::<u32>()) as isize;
            let data = &self.m_indicies[0] as *const u32 as *const c_void;
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);
    
            // set the vertex attribute pointers
            let size = size_of::<VertexPosNormTexTanBi>() as i32;
            // vertex Positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, offset_of!(VertexPosNormTexTanBi, location) as *const c_void);
            // vertex normals
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, size, offset_of!(VertexPosNormTexTanBi, normal) as *const c_void);
            // vertex texture coords
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, size, offset_of!(VertexPosNormTexTanBi, texcoord) as *const c_void);
            // vertex tangent
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE, size, offset_of!(VertexPosNormTexTanBi, tangent) as *const c_void);
            // vertex bitangent
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(4, 3, gl::FLOAT, gl::FALSE, size, offset_of!(VertexPosNormTexTanBi, bitangent) as *const c_void);
    
            gl::BindVertexArray(0);
        }
    }

    pub fn Draw(&self, shader:&Shader)
    {
        unsafe
        {
            // bind appropriate textures
            let mut diffuseNr  = 0;
            let mut specularNr = 0;
            let mut normalNr   = 0;
            let mut heightNr   = 0;
            for (i, texture) in self.m_textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32); // active proper texture unit before binding
                // retrieve texture number (the N in diffuse_textureN)
                let name = &texture.resource_name;
                let number = match name.as_str() {
                    "texture_diffuse" => {
                        diffuseNr += 1;
                        diffuseNr
                    },
                    "texture_specular" => {
                        specularNr += 1;
                        specularNr
                    }
                    "texture_normal" => {
                        normalNr += 1;
                        normalNr
                    }
                    "texture_height" => {
                        heightNr += 1;
                        heightNr
                    }
                    _ => panic!("unknown texture type")
                };
                // now set the sampler to the correct texture unit
                let sampler = CString::new(format!("{}{}", name, number)).unwrap();
                gl::Uniform1i(gl::GetUniformLocation(shader.ID, sampler.as_ptr()), i as i32);
                // and finally bind the texture
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }

            // draw mesh
            gl::BindVertexArray(self.m_vao);
            gl::DrawElements(gl::TRIANGLES, self.m_indicies.len() as i32, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);

            // always good practice to set everything back to defaults once configured.
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }
}
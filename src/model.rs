
#![allow(dead_code)]

extern crate glfw;
extern crate gl;

use crate::shader::Shader;
use crate::math::*;
use crate::rc_internal;
use crate::utils;


use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


use std::ptr;
use std::os::raw::c_void;
use std::ffi::{CString};
use std::time::{Instant};
use std::mem::size_of;
use std::str::Chars;
use crate::core::types::{RevTexture, VertexPosNormTexTanBi};

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
        RevModel {
            m_meshes : Vec::new(),
            m_textures : Vec::new(),
            m_shader : Shader::default(),
        }
    }

    pub fn new_from_path(base_path:&str, model_name:&str) -> Self
    {   
        // retr ieve the directory path of the filepath
        let mut model_instance = RevModel::default();

        let start_time = Instant::now();
        model_instance.m_shader = Shader::new(	
            utils::modify_to_resource_path("shaders/shader.vs").as_str(),	
            utils::modify_to_resource_path("shaders/shader.fs").as_str(),	
        ).unwrap_or_else(|error|{	
            panic!("Failed serializing shader: {} path: {}", error, utils::modify_to_resource_path("shaders/vs.vs"));	
        }); 

        let model_file = format!("{}{}", base_path, model_name);
        let path = Path::new(model_file.as_str());
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
                vertices.push(VertexPosNormTexTanBi::new(
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
                    let texture = 
                    rc_internal::load_material_texture(
                        &mut model_instance.m_textures,
                        format!("{}{}", base_path, material.diffuse_texture).as_str(),
                        utils::try_remove_file_extension_from_file(material.diffuse_texture.as_str(), ".").as_str(),
                        "texture_diffuse");
                    textures.push(texture);
                }
                // 2. specular map
                if !material.specular_texture.is_empty() {
                    let texture = rc_internal::load_material_texture(&mut model_instance.m_textures, 
                        format!("{}{}", base_path, material.specular_texture).as_str(),
                     utils::try_remove_file_extension_from_file(material.specular_texture.as_str(), ".").as_str(),
                         "texture_specular");
                    textures.push(texture);
                }
                // 3. normal map
                if !material.normal_texture.is_empty() {
                    let texture = rc_internal::load_material_texture(
                        &mut model_instance.m_textures, 
                        format!("{}{}", base_path,  material.normal_texture).as_str(),
                        utils::try_remove_file_extension_from_file(material.normal_texture.as_str(), ".").as_str(),
                        "texture_normal");
                    textures.push(texture);
                }
                // NOTE: no height maps
            }
            model_instance.m_meshes.push(RevMesh::new(vertices, indices, textures));
        }

        let end_time =  Instant::now();
        println!("load time context:loadtimeContext {} ms: {}", model_file.as_str(), end_time.saturating_duration_since(start_time).as_millis());
        model_instance
    }


    pub fn save_to_file(&self, base_path:&str, model_path:&str) {
        for texture in &self.m_textures {
            if texture.base_tex_id.is_empty() {
                println!("Will not save texture of resource {} path {} as empty", texture.resource_name, texture.path);
                continue
            }

            //build texture path
            let mut save_path: String = String::from(base_path);
            save_path.push_str(texture.base_tex_id.as_str() );
            save_path.push_str(".revtexture");

            // Open a file in write-only mode, returns `io::Result<File>`
            let mut file = match File::create(&save_path) {
                Err(why) => panic!("couldn't create {}: {}", save_path, why),
                Ok(file) => file,
            };

            let v =  bincode::serialize(&texture).unwrap();
            match file.write_all( &v) {
                Err(why) => panic!("couldn't write to {}: {}", save_path, why),
                Ok(_) => println!("successfully wrote to {}", save_path),
            }
        }
    }

    pub fn draw(&self) {
        for mesh in &self.m_meshes {
            mesh.draw(&self.m_shader);
        }
    }
}

impl RevMesh
{
    pub fn default() -> Self
    {
        RevMesh 
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
        let mut m = RevMesh
        {
            m_verts : verts,
            m_indicies : indicies,
            m_textures : textures,
            ..RevMesh::default()
        };

        m.setup_mesh();
        m
    }

    fn setup_mesh(&mut self)
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

    pub fn draw(&self, shader:&Shader)
    {
        unsafe
        {
            // bind appropriate textures
            let mut diffuse_nr  = 0;
            let mut specular_nr = 0;
            let mut normal_nr   = 0;
            let mut height_nr   = 0;
            for (i, texture) in self.m_textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32); // active proper texture unit before binding
                // retrieve texture number (the N in diffuse_textureN)
                let name = &texture.resource_name;
                let number = match name.as_str() {
                    "texture_diffuse" => {
                        diffuse_nr += 1;
                        diffuse_nr
                    },
                    "texture_specular" => {
                        specular_nr += 1;
                        specular_nr
                    }
                    "texture_normal" => {
                        normal_nr += 1;
                        normal_nr
                    }
                    "texture_height" => {
                        height_nr += 1;
                        height_nr
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
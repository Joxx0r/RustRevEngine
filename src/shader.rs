#![allow(non_snake_case)]
use std::ffi::{CString};
use std::ptr;
use std::str;
use gl;
use gl::types::*;

use std::error::Error;

use crate::utils;

pub struct Shader {
    pub ID : u32,
}

impl Shader
{
    pub fn new(vertexPath:&str, fragmentPath:&str) -> Result<Shader, Box<dyn Error>>
    {
        let mut shader = Shader { ID: 0 };
        let vertexContent = utils::read_username_from_file(vertexPath)?;
        let fragmentContent = utils::read_username_from_file(fragmentPath)?;

        let vShaderCode = CString::new(vertexContent.as_bytes()).unwrap();
        let fShaderCode = CString::new(fragmentContent.as_bytes()).unwrap();

        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.checkCompileErrors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.checkCompileErrors(fragment, "FRAGMENT");
            // shader Program
            let ID = gl::CreateProgram();
            gl::AttachShader(ID, vertex);
            gl::AttachShader(ID, fragment);
            gl::LinkProgram(ID);
            shader.checkCompileErrors(ID, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.ID = ID;
        }
        
        Ok(shader)
    }

    unsafe fn checkCompileErrors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut infoLog = Vec::with_capacity(1024);
        infoLog.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&infoLog).unwrap());
            }

        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&infoLog).unwrap());
            }
        }

    }

    pub unsafe fn useProgram(&self) {
        gl::UseProgram(self.ID)
    }


}
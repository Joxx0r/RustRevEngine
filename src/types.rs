
#[allow(dead_code)]

use std::error::Error;
use std::fmt;

use crate::math::*;

pub struct RevColor
{
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32,
}

#[allow(dead_code)]
impl RevColor
{
    fn new(r:f32, g:f32, b:f32, a:f32) -> RevColor
    {
        RevColor{r:r, g:g, b:b, a:a}
    }

    pub fn red() -> RevColor
    {
        RevColor::new(1.0, 0.0, 0.0, 1.0)
    }
    pub fn black() -> RevColor
    {
        RevColor::new(0.0, 0.0, 0.0, 1.0)
    }
    pub fn white() -> RevColor
    {
        RevColor::new(1.0, 1.0, 1.0, 1.0)
    }
}

// settings
pub struct StartupSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub title : String,
}

pub fn create_startup_settings(width:u32, height:u32) -> StartupSettings {
    let f = StartupSettings{
        window_width: width,
        window_height: height,
        title: String::from("RevEngine"),
    };
    f
}

#[derive(Debug)]
pub struct RevError 
{
    display:String,
}

impl RevError 
{
    #[allow(dead_code)]
    pub fn new(name:&str) -> RevError {
        RevError{ display: String::from(name)}
    }
}

impl Error for RevError
{
    fn description(&self) -> &str {
        self.display.as_str()
    }
}

impl fmt::Display for RevError 
{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "SuperErrorSideKick is here!")
    }
}

#[allow(dead_code)]
pub struct VertexPosColTex
{
    pub location : Vec3,
    pub color : Vec3,
    pub texcoord : Vec2,
}

#[allow(dead_code)]
impl VertexPosColTex 
{
   fn default() -> Self {
    Self {
            location: Vec3::new_1(0.0),
            color: Vec3::new_1(0.0),
            texcoord: Vec2::new_1(0.0),
        }
    }

    pub fn new(in_location: Vec3,in_color: Vec3, in_tex_coord: Vec2 ) -> Self {
        VertexPosColTex{location:in_location, color:in_color, texcoord:in_tex_coord}
    }
}

pub struct VertexPosNormTexTanBi
{
    pub location : Vec3,
    pub normal : Vec3,
    pub texcoord : Vec2,
    pub tangent : Vec3,
    pub bitangent : Vec3,
}

#[allow(dead_code)]
impl VertexPosNormTexTanBi 
{
    fn default() -> Self {
        Self {
                location: Vec3::new_1(0.0),
                normal: Vec3::new_1(0.0),
                texcoord: Vec2::new_1(0.0),
                tangent: Vec3::new_1(0.0),
                bitangent: Vec3::new_1(0.0)
            }
       }
        
    pub fn new(in_location: Vec3,in_normal: Vec3, in_texcoord: Vec2, in_tangent:Vec3, in_bitangent:Vec3 ) -> Self
    {
        VertexPosNormTexTanBi{location:in_location, normal:in_normal, texcoord:in_texcoord, tangent:in_tangent, bitangent:in_bitangent }
    }
}
#[derive(Clone)]
pub struct RevTexture {
    pub id: u32,
    pub resource_name: String,
    pub path: String,
}
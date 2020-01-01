#![allow(dead_code)]

use std::ops::Mul;

#[derive(Debug, PartialEq)]
pub struct Vec2 {
    pub x : f32,
    pub y : f32,
}

impl Vec2 {
    pub fn default() -> Self{  
        Vec2::new_1(0.0)
    }
    pub fn zero() -> Self{  
        Vec2::new_1(0.0)
    }
    pub fn new() -> Vec2{  
        Vec2{x:0.0, y:0.0 }
    }
    pub fn new_1(in_s:f32) -> Vec2 {   
        Vec2{x:in_s, y:in_s}
    }   
    pub fn new_2(in_x:f32, in_y:f32) -> Vec2 {   
        Vec2{x:in_x, y:in_y }
    }
}

#[derive(Debug, PartialEq)]
pub struct Vec3{
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Vec3 {
    pub fn default() -> Self {  
        Vec3::new_1(0.0)
    }
    pub fn zero() -> Self {  
        Vec3::new_1(0.0)
    }
    pub fn new() -> Vec3 {  
        Vec3{x:0.0, y:0.0, z:0.0}
    }
    pub fn new_1(in_s:f32) -> Vec3 {   
        Vec3{x:in_s, y:in_s, z:in_s}
    }   
    pub fn new_3(in_x:f32, in_y:f32, in_z:f32) -> Vec3 {   
        Vec3{x:in_x, y:in_y, z:in_z}
    }
}

#[derive(Debug, PartialEq)]
pub struct Vec4{
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32,
}

impl Vec4{
    pub fn default() -> Self{  
        Vec4::new_1(0.0)
    }
    pub fn zero() -> Self{  
        Vec4::new_1(0.0)
    }
    pub fn new() -> Vec4{  
        Vec4{x:0.0, y:0.0, z:0.0, w:0.0}
    }
    pub fn new_1(in_s:f32) -> Vec4{   
        Vec4{x:in_s, y:in_s, z:in_s, w:1.0}
    } 
    pub fn new_3(in_x:f32, in_y:f32, in_z:f32) -> Vec4{   
        Vec4{x:in_x, y:in_y, z:in_z, w:1.0}
    }
    pub fn new_4(in_x:f32, in_y:f32, in_z:f32, in_w:f32) -> Vec4{   
        Vec4{x:in_x, y:in_y, z:in_z, w:in_w}
    }
}

impl Mul for Vec2 {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self {
       Vec2::new_2(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul for Vec3 {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self {
       Vec3::new_3(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul for Vec4 {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec4::new_4(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
     }
}

use gl;
use crate::types::{RevColor, RevTexture};

use std::os::raw::c_void;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use std::time::{Instant};

use std::path::Path;

pub fn clear_color_gl(color:RevColor)
{
    unsafe
    {
        gl::ClearColor(color.r, color.g, color.b, color.a);
    }
}

pub unsafe fn texture_from_file(path: &str) -> (u32, Vec<u8>, i32, i32, i32) {
    let filename = path;
    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);

    let img = image::open(&Path::new(&filename)).expect(format!("Failed loading texture {}", path).as_str());
    let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
    };

    let end_time = Instant::now();
    let data = img.raw_pixels();
    (texture_id, data, format as i32, img.width() as i32, img.height() as i32)
}

pub unsafe fn texture_to_gl(texture:&RevTexture ) {

    gl::BindTexture(gl::TEXTURE_2D, texture.id);
    gl::TexImage2D(gl::TEXTURE_2D, 0, texture.format, texture.width, texture.height,
                   0, texture.format as u32, gl::UNSIGNED_BYTE, &texture.raw_data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
}


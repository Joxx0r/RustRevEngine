
use crate::rc;
use crate::types::*;
use std::time::{Instant};

pub fn load_material_texture(exisiting_textures:&mut Vec<RevTexture>, path: &str, tex_id: &str, type_name: &str) -> RevTexture {

    let start_time = Instant::now();
    let texture = exisiting_textures.iter().find(|t| t.path == path);
    if let Some(texture) = texture {
        return texture.clone();
    }

    let texture_load_result =  unsafe {
        rc::texture_from_file(path)
    };
    let texture = RevTexture {
        id: texture_load_result.0,
        format: texture_load_result.2,
        width: texture_load_result.3,
        height: texture_load_result.4,
        resource_name: type_name.into(),
        path: path.into(),
        raw_data: texture_load_result.1,
        base_tex_id: tex_id.into(),
    };
    unsafe {
        rc::texture_to_gl(&texture);
    }

    exisiting_textures.push(texture.clone());
    let end_time = Instant::now();
    
    println!("Loading texture: {} took (ms) {} ", path, end_time.saturating_duration_since(start_time).as_millis());
    texture
}

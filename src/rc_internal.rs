
use crate::rc;
use crate::types::*;
use std::time::{Instant};

pub fn load_material_texture(exisiting_textures:&mut Vec<RevTexture>, path: &str, tex_id: &str, type_name: &str) -> RevTexture {

    let start_time = Instant::now();
    let texture = exisiting_textures.iter().find(|t| t.path == path);
    if let Some(texture) = texture {
        return texture.clone();
    }

    let textureLoadResult =  unsafe { rc::texture_from_file(path) };

    let texture = RevTexture {
        id: textureLoadResult.0,
        resource_name: type_name.into(),
        path: path.into(),
        raw_data: textureLoadResult.1,
        base_tex_id: tex_id.into(),
    };
    exisiting_textures.push(texture.clone());

    let end_time = Instant::now();
    
    println!("Loading texture: {} took (ms) {} ", path, end_time.saturating_duration_since(start_time).as_millis());
    texture
}


use crate::rc;
use crate::types::*;
use std::time::{Instant};

pub fn load_material_texture(exisiting_textures:&mut Vec<RevTexture>, path: &str, type_name: &str) -> RevTexture {
   
    let start_time = Instant::now();
    let texture = exisiting_textures.iter().find(|t| t.path == path);
    if let Some(texture) = texture {
        return texture.clone();
    }

    let texture = RevTexture {
        id: unsafe { rc::texture_from_file(path) },
        resource_name: type_name.into(),
        path: path.into()
    };
    exisiting_textures.push(texture.clone());

    let end_time = Instant::now();
    
    println!("Loading texture: {} took (ms) {} ", path, end_time.saturating_duration_since(start_time).as_millis());
    texture
}

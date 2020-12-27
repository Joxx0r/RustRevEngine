
use crate::{rc, utils};
use std::time::{Instant};
use std::fs;
use crate::core::types::RevTexture;

pub fn load_material_texture(exisiting_textures:&mut Vec<RevTexture>, path: &str, tex_id: &str, type_name: &str) -> RevTexture {

    let start_time = Instant::now();
    let texture = exisiting_textures.iter().find(|t| t.path == path);
    if let Some(texture) = texture {
        return texture.clone();
    }

    let mut texture = RevTexture::default();
    let mut modified_path = utils::try_remove_file_extension_from_file(path, ".");
    modified_path.push_str(".revtexture");
    let texture_id = unsafe {
            rc::generate_id()
    };

    let start_time_load_textures = Instant::now();
    if utils::file_exists(modified_path.as_str())
    {
        let binary_data = fs::read(&modified_path).expect("unable to read metadata");
        texture = bincode::deserialize(&binary_data).unwrap();
        texture.id = texture_id;
    }
    else
    {
        let texture_load_result =  unsafe {
            rc::texture_from_file(path)
        };

        texture = RevTexture {
            id: texture_id,
            format: texture_load_result.1,
            width: texture_load_result.2,
            height: texture_load_result.3,
            resource_name: type_name.into(),
            path: path.into(),
            raw_data: texture_load_result.0,
            base_tex_id: tex_id.into(),
        };
    }

    let end_time_load_textures = Instant::now().saturating_duration_since(start_time_load_textures);
    let start_time_gl_textures = Instant::now();
    unsafe {
        rc::texture_to_gl(&texture);
    }

    exisiting_textures.push(texture.clone());

    let end_time = Instant::now();
    let end_time_textures = end_time.saturating_duration_since(start_time_gl_textures);
    println!("Total loading texture: {} took (ms) {} gl texture setting {} (ms) loading texture from file {} (ms)", path, end_time.saturating_duration_since(start_time).as_millis(), end_time_textures.as_millis(), end_time_load_textures.as_millis() );
    texture
}

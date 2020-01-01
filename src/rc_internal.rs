
use crate::rc;
use crate::types::*;
use crate::utils;

pub fn load_material_texture(exisiting_textures:&mut Vec<RevTexture>, path: &str, type_name: &str) -> RevTexture {
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
    texture
}

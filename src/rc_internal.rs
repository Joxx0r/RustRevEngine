
use crate::rc;
use crate::types::*;
use crate::utils;

pub fn load_material_texture(exisiting_textures:&mut Vec<RevTexture>, path: &str, typeName: &str) -> RevTexture {
    let texture = exisiting_textures.iter().find(|t| t.path == path);
    if let Some(texture) = texture {
        return texture.clone();
    }

    let texture = RevTexture {
        id: unsafe { rc::texture_from_file(utils::modify_to_resource_path(path).as_str()) },
        resource_name: typeName.into(),
        path: path.into()
    };
    exisiting_textures.push(texture.clone());
    texture
}

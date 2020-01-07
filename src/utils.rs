

#![macro_use]

use std::io;
use std::io::Read;
use std::fs::File;

pub fn read_content_from_file(path:&str) -> Result<String, io::Error> 
{
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

const SRC_RESOURCE_PATH:&str = "../../resources/";

pub fn modify_to_resource_path(from_resource:&str) -> String
{
    let mut s = String::from(SRC_RESOURCE_PATH);
    s.push_str(from_resource);
    s
}

#[allow(dead_code)]
pub fn try_remove_file_extension_from_file(from_resource:&str, extension:&str) -> String
{
    let s = String::from(from_resource);
    let substr : String = match s.find(extension) {
        Some(elem) => {
            String::from(&s[..elem])
        }
        None => s
    };
    substr
}

/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

/// Get offset to struct member, similar to `offset_of` in C/C++
/// From https://stackoverflow.com/questions/40310483/how-to-get-pointer-offset-in-bytes/40310851#40310851
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize
    }
}



use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

pub fn read_username_from_file(path:&str) -> Result<String, io::Error> 
{
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

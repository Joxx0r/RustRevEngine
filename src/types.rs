
use std::error::Error;
use std::fmt;

pub struct RevColor
{
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32,
}

#[allow(dead_code)]
impl RevColor
{
    fn new(r:f32, g:f32, b:f32, a:f32) -> RevColor
    {
        RevColor{r:r, g:g, b:b, a:a}
    }

    pub fn red() -> RevColor
    {
        RevColor::new(1.0, 0.0, 0.0, 1.0)
    }
    pub fn black() -> RevColor
    {
        RevColor::new(0.0, 0.0, 0.0, 1.0)
    }
    pub fn white() -> RevColor
    {
        RevColor::new(1.0, 1.0, 1.0, 1.0)
    }
}


// settings
pub struct StartupSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub title : String,
}

pub fn create_startup_settings(width:u32, height:u32) -> StartupSettings {
    let f = StartupSettings{
        window_width: width,
        window_height: height,
        title: String::from("RevEngine"),
    };
    f
}

#[derive(Debug)]
pub struct RevError 
{
    display:String,
}

impl RevError 
{
    #[allow(dead_code)]
    pub fn new(name:&str) -> RevError {
        RevError{ display: String::from(name)}
    }
}

impl Error for RevError
{
    fn description(&self) -> &str {
        self.display.as_str()
    }
}

impl fmt::Display for RevError 
{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "SuperErrorSideKick is here!")
    }
}
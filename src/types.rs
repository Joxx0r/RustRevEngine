
pub const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

pub struct RevColor
{
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32,
}

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
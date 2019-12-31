
use gl;
use crate::types::RevColor;

pub fn clear_color_gl(color:RevColor)
{
    unsafe
    {
        gl::ClearColor(color.r, color.g, color.b, color.a);
    }
}

#version 330 core
out vec4 frag_color;

in vec2 our_texcoord;

uniform sampler2D texture_diffuse;
uniform sampler2D texture_normal;

void main()
{
   frag_color = texture(texture_diffuse, our_texcoord);
}
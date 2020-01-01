#version 330 core
out vec4 frag_color;

in vec2 our_texcoord;

uniform sampler2D texture_1;
uniform sampler2D texture_2;

void main()
{
   frag_color = mix(texture(texture_1, our_texcoord), texture(texture_2, our_texcoord), 0.2);
}
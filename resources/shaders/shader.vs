#version 330 core
layout (location = 0) in vec3 pos_1;
layout (location = 1) in vec3 color_1;
layout (location = 2) in vec2 texcoord_1;

out vec3 our_color;
out vec2 our_texcoord;

void main()
{
    gl_Position = vec4(pos_1, 1.0);
    our_color = color_1;
    our_texcoord = texcoord_1;
}
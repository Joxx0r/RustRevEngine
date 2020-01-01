#version 330 core
layout (location = 0) in vec3 pos_1;
layout (location = 1) in vec3 norm_1;
layout (location = 2) in vec2 texcoord_1;
layout (location = 3) in vec3 tangent_1;
layout (location = 4) in vec3 bitangent_1;

out vec2 our_texcoord;

void main()
{
    gl_Position = vec4(pos_1, 1.0);
    our_texcoord = texcoord_1;
}
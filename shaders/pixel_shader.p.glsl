// #version 430 core

// in vec4 vtxColor;
// out vec4 FragColor;

// void main()
// {
// 	FragColor = vtxColor; 
// }

#version 430 core
out vec4 FragColor;

uniform vec4 vtx_color;

void main()
{
    FragColor = vtx_color;
} 
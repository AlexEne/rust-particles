// #version 430 core

// in vec4 vtxColor;
// out vec4 FragColor;

// void main()
// {
// 	FragColor = vtxColor; 
// }

#version 330 core
out vec4 FragColor;

uniform vec4 vtxColor;

void main()
{
    FragColor = vtxColor;
} 
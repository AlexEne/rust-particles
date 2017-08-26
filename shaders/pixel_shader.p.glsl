// #version 430 core

// in vec4 vtxColor;
// out vec4 FragColor;

// void main()
// {
// 	FragColor = vtxColor; 
// }

#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.3f, 0.2f, 0.3f);
} 
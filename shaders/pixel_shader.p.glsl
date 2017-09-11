#version 430 core

layout (location = 0) out vec4 FragColor;
layout (location = 1) out vec4 Highlights;

in vec4 vtxColor;

void main()
{
    FragColor = vtxColor;
    
    vec3 limit_intensity = vec3(0.9);
    if (limit_intensity.r < vtxColor.r
        && limit_intensity.g < vtxColor.g
        && limit_intensity.b < vtxColor.b
    ) 
    {
         Highlights = FragColor;
    }    
} 
#version 430 core

layout (location = 0) out vec4 FragColor;
in vec2 TexCoords;

uniform sampler2D screenTexture;

uniform int vertical;
uniform float weight[5] = float[](0.2, 0.17, 0.12, 0.1, 0.05);

void main()
{
    vec3 color = texture(screenTexture, TexCoords).rgb * weight[0];
    vec2 pixel_size = 1.0/textureSize(screenTexture, 0);
    if(vertical == 1)
    {
        for(int i = 1; i < 5; i++)
        {
             color += texture(screenTexture, TexCoords + vec2(0.0, pixel_size.y*i)).rgb * weight[i];
             color += texture(screenTexture, TexCoords - vec2(0.0, pixel_size.y*i)).rgb * weight[i];
        }
    }
    else
    {
        for(int i = 1; i < 5; i++)
        {
            color += texture(screenTexture, TexCoords + vec2(pixel_size.x*i, 0.0)).rgb * weight[i];
            color += texture(screenTexture, TexCoords - vec2(pixel_size.x*i, 0.0)).rgb * weight[i];
        }

    }

    FragColor = vec4(color.rgb, 1.0);
} 
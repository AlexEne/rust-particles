#version 330 core
out vec4 FragColor;
  
in vec2 TexCoords;

uniform sampler2D screenTexture;

void main()
{ 
    vec3 color = texture(screenTexture, TexCoords).rgb;
    color *= 2; //add some hardcoded exposure
    vec3 tone_mapped_color = color / (color + 1);

    vec3 gamma_corrected_color = pow(tone_mapped_color, vec3(1.0/2.2));
    FragColor = vec4(gamma_corrected_color.rgb, 1.0);
}
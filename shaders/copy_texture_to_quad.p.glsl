#version 330 core
out vec4 FragColor;
  
in vec2 TexCoords;

uniform sampler2D screenTexture;
uniform sampler2D bloom;
void main()
{ 
    vec3 color = texture(screenTexture, TexCoords).rgb;
    vec3 bloom_color = texture(bloom, TexCoords).rgb * 0.2;
    color += bloom_color;
    
    float exposure = 2.0;
    color *= exposure; //add some hardcoded exposure
    //vec3 tone_mapped_color = color / (color + 1);
    vec3 tone_mapped_color = vec3(1.0) - exp(-color * exposure);

    vec3 gamma_corrected_color = pow(tone_mapped_color, vec3(1.0/2.2));
    FragColor = vec4(gamma_corrected_color.rgb, 1.0);
}
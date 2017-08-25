/*
#version 430 core

//Just saying it again :) position.w holds the particle speed.
in layout (location = 0) vec4 position; 
in layout (location = 1) vec3 color;

uniform mat4 ViewMtx;
uniform mat4 ProjMtx;

out vData
{
    vec4 transformedColor;
} v_color;


void main()
{
	vec4 viewPos = ViewMtx * vec4(position.xyz, 1.0);
	gl_Position = ProjMtx * viewPos;
	
	//Get the speed and interpolate (mix) between largeSpeed and smallSpeed color.
	float speed = clamp(position.w, 0.0, 400.0);
	vec3 largeSpeed = vec3(0.4, 0.4, 0.4);
	vec3 smallSpeed = vec3(1.0, 1.0, 1.0);

	//Interpolate between the two colors
	vec3 clr = mix(smallSpeed, largeSpeed, vec3(speed/200.0, speed/200.0, speed/200.0));
	
	v_color.transformedColor = vec4(clr, (viewPos.z+3000)/3000);
}
*/

#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
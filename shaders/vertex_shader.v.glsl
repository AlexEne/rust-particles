/*
#version 430 core

//Just saying it again :) position.w holds the particle speed.
in layout (location = 0) vec4 position; 
in layout (location = 1) vec3 color;

uniform mat4 ViewMtx;
uniform mat4 ProjMtx;




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

#version 430 core
out vData
{
    vec4 transformedColor;
} v_color;

layout (location = 0) in vec4 position;

uniform mat4 view_from_world;
uniform mat4 proj_from_view;

void main()
{
    vec4 viewPos = view_from_world * vec4(position.xyz, 1.0);
    gl_Position = proj_from_view * viewPos;
    v_color.transformedColor = vec4(0.0, 1.0, 1.0, 1.0);
}
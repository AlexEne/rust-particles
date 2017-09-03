#version 430

//Size of compute shader local work group - x=32, y=32, z=1(default)
layout( local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

layout ( binding = 0 ) buffer
buffer_InPos
{
	vec4	InPos[];
};

layout ( binding = 1 ) buffer
buffer_InVelocity
{
	vec4	InVelocity[];
};

layout ( binding = 2 ) buffer
buffer_OutPos
{
	vec4	OutPos[];
};

layout ( binding = 3 ) buffer
buffer_OutVelocity
{
	vec4	OutVelocity[];
};

// layout( binding = 2, rgba32f) uniform image2D inVelocity;
// layout( binding = 3, rgba32f) uniform image2D outVelocity;

uniform float dt;

const int spheresCount = 20;
uniform	vec3 sphereOffsets[spheresCount];
uniform	float sphereRadius[spheresCount];

uniform int g_NumParticles;

const float gAccel = 9.8;

//Samples the distance field and returns the value for point p
//For the moment just use a distance function.
float DistanceFieldCircle(vec3 p, vec3 sphereOffset, float sphereRadius)
{
	return length(sphereOffset - p) - sphereRadius;
}

float rand(vec2 co){
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453) * 1000.0;
}

void main(void)
{
	uint index = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y * gl_NumWorkGroups.x * gl_WorkGroupSize.x;

	if(index > g_NumParticles)
		return;

	vec4 particlePos = InPos[index];
    vec4 particleVelocity = InVelocity[index];

	//Update Velocity
	vec4 newParticleVelocity = particleVelocity;
	
	// We could recycle this particle since it's life is over. ( not used now )
	// For now, recycle the particle if we touched the ground of if our speed is too small.
	if ( particlePos.y <= 0.001 )
	{
		float rand1 = rand(particlePos.xz);
		float rand2 = rand(particlePos.zx);
		//Generate a random possition
		particlePos.x = -700.0 + mod(rand1 * 10, 1400.0);
		particlePos.y =	 500.0 + mod(rand1, 50);
		particlePos.z = -700.0 + mod(rand2 * 10, 1400.0);
		
		//Generate a random speed.
		newParticleVelocity.x = -5.0 + mod(rand2 * 100.0f, 10.0);
		newParticleVelocity.y = -5.0 + mod(rand2 * 100.0f, 10.0);
		newParticleVelocity.z = -5.0 + mod(rand1 * 100.0f, 10.0);
		newParticleVelocity.w = 10.0;
	}
	else
	{
		//Just update the particle
		particlePos.xyz = particlePos.xyz + particleVelocity.xyz*dt;
		newParticleVelocity.y = newParticleVelocity.y - gAccel * dt;
	}

	//Collisions
	if (false){
		//Ground Bounce
		if(particlePos.y <= 0.01)
		{
			newParticleVelocity.xyz *= 0.2;
			newParticleVelocity.y *= -1.0;
		}

		//Walls
		const float wallDamping = 0.5;
		if(particlePos.x > 700) 
		{
			particlePos.x = 699.9;
			newParticleVelocity.x *= -wallDamping;
		}
		else if (particlePos.x < -700)
		{
			particlePos.x = -699.9;
			newParticleVelocity.x *= -wallDamping;
		}
		else if(particlePos.z > 700)
		{
			particlePos.z = 699.9;
			newParticleVelocity.z *= -wallDamping;
		}
		else if(particlePos.z < -700)
		{
			particlePos.z = -699.9;
			newParticleVelocity.z *= -wallDamping;
		}


		//Try to find the closest sphere to our particle.
		float minDist = 10000.0;
		int closestSphereIdx = -1;
		for(int i = 0; i < spheresCount; i++)
		{			
			//Sphere_t sphere = ;
			vec3 sphereCenter = sphereOffsets[i];
			vec3 localPosition = particlePos.xyz - sphereCenter;

			//Distance field evaluation
			float dist = DistanceFieldCircle(particlePos.xyz, sphereCenter, sphereRadius[i]);
		
			if(dist < minDist && dist < 0)
			{
				minDist = dist;
				closestSphereIdx = i;
			}
		}

		if (closestSphereIdx != -1)
		{
			//We we collided with a sphere.

			//Compute the reflection vector
			//Sphere_t closestSphere = spheres[closestSphereIdx];
			vec3 localPosition = vec3(particlePos.x, particlePos.y, particlePos.z) - sphereOffsets[closestSphereIdx];
			vec3 ReflectionNormal = normalize(localPosition);
			
			//Reflect our speed
			newParticleVelocity.xyz = reflect(newParticleVelocity.xyz,ReflectionNormal);
			if(abs(newParticleVelocity.x) > 2.0)
                newParticleVelocity.x *= 0.2;
            if(abs(newParticleVelocity.y) > 2.0)
                newParticleVelocity.y *= 0.2;
            if(abs(newParticleVelocity.z) > 2.0)
                newParticleVelocity.z *= 0.2;

			//Move the particle away from the collision just a bit.
			particlePos.xyz = sphereOffsets[closestSphereIdx] + ReflectionNormal * ( sphereRadius[closestSphereIdx] + 0.1);
		}
	}

	//SetColor based on the velocity
	particlePos.w = length(newParticleVelocity);

	//Update our remaining life
	//newParticleVelocity.w -= dt;

	//Save the new possitions and velocities
	InPos[index] = particlePos;
    InVelocity[index] = newParticleVelocity;
}
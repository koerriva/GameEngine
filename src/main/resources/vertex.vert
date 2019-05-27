#version 330

layout (location=0) in vec3 position;
layout (location=1) in vec3 colour;

out vec3 outColour;

void main()
{
    gl_Position = vec4(position, 1.0);
    outColour = colour;
}
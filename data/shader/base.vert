#version 330

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 texcoord;

uniform float time;

uniform mat4 P;
uniform mat4 V;
uniform mat4 M;

void main(){
    gl_Position = P*V*M*vec4(position,1.0);
}
#version 330

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 texcoord;

uniform float time;

void main(){
    gl_Position = vec4(position*abs(sin(time)),1.0);
}
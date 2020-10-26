#version 330

out vec4 fragColor;

uniform float time;

void main(){
    float r = abs(sin(time));
    fragColor = vec4(r,0.2,0.4,1.0);
}
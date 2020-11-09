#version 330

out vec4 FragColor;

uniform float time;

in vec2 v_TexCoord;
in vec3 v_Color;
uniform sampler2D texture0;

void main(){
    float r = abs(sin(time));
    FragColor = vec4(v_Color,1.0);
}
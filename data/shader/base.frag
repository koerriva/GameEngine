#version 330

out vec4 FragColor;

uniform float time;

in vec2 v_TexCoord;
uniform sampler2D texture0;

void main(){
    float r = abs(sin(time));
    FragColor = texture(texture0,v_TexCoord);
}
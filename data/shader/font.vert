#version 330

layout(location=0) vec3 pos;
layout(location=2) vec2 texcoord;

out vec2 v_TexCoord;
uniform mat4 P;
void main() {
    gl_Position = P*vec4(pos, 1.0);
    v_TexCoord = texcoord;
}

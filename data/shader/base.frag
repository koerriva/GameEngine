#version 330

out vec4 FragColor;

in vec2 v_TexCoord;
in vec3 v_Color;
uniform sampler2D base_color_sampler;
uniform sampler2D metallic_roughness_sampler;

void main(){
    vec4 base_color = texture(base_color_sampler,v_TexCoord);
    vec4 metallic_roughness_color = texture(metallic_roughness_sampler,v_TexCoord);
    FragColor = base_color;
}
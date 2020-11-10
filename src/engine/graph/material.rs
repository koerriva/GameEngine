use crate::engine::graph::texture::Texture;
use crate::engine::graph::shader::ShaderProgram;
use nalgebra::clamp;
use crate::engine::camera::Camera;
use nalgebra_glm::TMat4;

pub struct Material{
    textures:Vec<Texture>,
    shader:ShaderProgram
}

impl Material {
    pub fn new(textures:Vec<Texture>,shader:ShaderProgram)->Self{
        Material{textures,shader}
    }

    pub fn bind(&self){
        self.shader.bind();
        let size = clamp(self.textures.len(),0,16);
        if size>0{
            unsafe {
                for i in 0..size {
                    gl::ActiveTexture(gl::TEXTURE0+i as u32);
                    let texture = &self.textures[i];
                    gl::BindTexture(gl::TEXTURE_2D, texture.id);
                }
            }
        }
    }

    pub fn unbind(&self){
        self.shader.unbind()
    }

    pub fn set_pvm(&self,p:&TMat4<f32>,v:&TMat4<f32>,m:&TMat4<f32>){
        self.shader.set_mat4("P",p);
        self.shader.set_mat4("V",v);
        self.shader.set_mat4("M",m)
    }
}
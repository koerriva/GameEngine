use crate::engine::graph::texture::Texture;
use crate::engine::graph::shader::ShaderProgram;
use nalgebra::clamp;

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
        let size = clamp(self.textures.len(),1,16);
        unsafe {
            for i in 0..size {
                gl::ActiveTexture(gl::TEXTURE0+i as u32);
                let texture = &self.textures[i];
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }
        }
    }

    pub fn unbind(&self){
        self.shader.unbind()
    }
}
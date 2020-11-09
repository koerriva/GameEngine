use crate::engine::graph::mesh::Mesh;
use crate::engine::graph::material::Material;
use crate::engine::camera::Camera;

pub struct Model{
    meshes:Vec<Mesh>,
    material:Material
}

impl Model{
    pub fn new(meshes:Vec<Mesh>,material:Material)->Self{
        Model{meshes,material}
    }

    pub fn attach_mesh(&mut self,mesh:Mesh){
        self.meshes.push(mesh)
    }

    pub fn draw(&self,camera:&Camera){
        self.material.bind();
        for mesh in &self.meshes {
            mesh.draw()
        }
        self.material.unbind();
    }
}
use crate::engine::graph::mesh::Mesh;
use crate::engine::graph::material::Material;
use crate::engine::camera::Camera;
use nalgebra_glm::TMat4;

pub struct Model{
    meshes:Vec<Mesh>,
    material:Material,
    transform:TMat4<f32>
}

impl Model{
    pub fn new(meshes:Vec<Mesh>,material:Material,transform:TMat4<f32>)->Self{
        Model{meshes,material,transform}
    }

    pub fn attach_mesh(&mut self,mesh:Mesh){
        self.meshes.push(mesh)
    }

    pub fn draw(&self,camera:&Camera){
        self.material.bind();
        self.material.set_pvm(&camera.projection_matrix(),&camera.view_matrix(),&self.transform);
        for mesh in &self.meshes {
            mesh.draw()
        }
        self.material.unbind();
    }
}
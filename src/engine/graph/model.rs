use crate::engine::graph::mesh::{Mesh, VertexAttr};
use crate::engine::graph::material::Material;
use crate::engine::camera::Camera;
use nalgebra_glm::{TMat4, TVec3, rotate, radians, TVec1, scale, vec3, quat_euler_angles, quat};
use std::f32::consts::PI;
use std::path::Path;
use crate::engine::graph::light::Light;
use gltf::Semantic;
use crate::engine::graph::shader::ShaderProgram;
use gltf::json::accessor::Type;
use gltf::camera::Projection::Orthographic;

pub struct Model{
    meshes:Vec<Mesh>,
    material:Material,
    pub transform:TMat4<f32>
}

pub struct Scene{
    pub camera:Camera,
    pub models:Vec<Model>
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

    pub fn rotate(&mut self,degrees:f32,axis:&TVec3<f32>){
        self.transform = rotate(&self.transform,degrees*PI/180.0,axis)
    }

    pub fn scale(&mut self,factor:f32){
        self.transform = scale(&self.transform,&vec3(factor,factor,factor))
    }
}

impl Scene {
    pub fn from_gltf(path:&str)->Scene{
        let mut models = Vec::new();
        let mut camera = Camera::new(4.0/3.0);
        let base_shader = ShaderProgram::new("base");

        let (document,buffers,images) = gltf::import(path)
            .expect("解析GLTF文件失败");
        for scene in document.scenes() {
            for node in scene.nodes() {
                //提取网格
                if node.mesh().is_some() {
                    let mesh_node = node.mesh().unwrap();
                    println!("mesh {:?}",mesh_node);
                    let mut meshes = Vec::new();
                    let textures = Vec::new();

                    for primitive in mesh_node.primitives() {
                        let mut indices = Vec::new();
                        let accessor = primitive.indices().unwrap();
                        let view = accessor.view().unwrap();
                        let buffer_idx = view.buffer().index();
                        let buffer = buffers.get(buffer_idx).unwrap();
                        let data_type = accessor.data_type();
                        let count = accessor.count();
                        println!("indices data type {:?},count {}",data_type,count);
                        let mut buf = vec![0;view.length()];
                        buf.copy_from_slice(&buffer[view.offset()..view.offset()+view.length()]);
                        let p = buf.as_mut_ptr() as *mut u16;
                        let slice = unsafe {std::slice::from_raw_parts(p,count)};
                        indices.extend([0].iter().cycle().take(count));
                        indices.copy_from_slice(slice);

                        let mut vertex_attr = VertexAttr{
                            position:Vec::new(),
                            normal:Vec::new(),
                            tex_coord:Vec::new(),
                            color:Vec::new()
                        };

                        for (semantic,accessor) in primitive.attributes() {
                            let view = accessor.view().unwrap();
                            let buffer_idx = view.buffer().index();
                            let buffer = buffers.get(buffer_idx).unwrap();
                            let data_type = accessor.data_type();
                            let count = accessor.count();

                            println!("attr data type {:?},view len:{:?}",data_type,view.length());
                            let mut buf = vec![0;view.length()];
                            buf.copy_from_slice(&buffer[view.offset()..view.offset()+view.length()]);
                            let p = buf.as_mut_ptr() as *mut f32;

                            match semantic {
                                Semantic::Positions => {
                                    let count = count*3;
                                    let slice = unsafe {std::slice::from_raw_parts(p,count)};
                                    vertex_attr.position.extend([0.0].iter().cycle().take(count));
                                    vertex_attr.position.copy_from_slice(slice);
                                    println!("Position total {},need {},offset {},count {}",buffer.len(),view.length(),view.offset(),count)
                                },
                                Semantic::Normals=>{
                                    let count = count*3;
                                    let slice = unsafe {std::slice::from_raw_parts(p,count)};
                                    vertex_attr.normal.extend([0.0].iter().cycle().take(count));
                                    vertex_attr.normal.copy_from_slice(slice);
                                    println!("Normal total {},need {},offset {},count {}",buffer.len(),view.length(),view.offset(),count)
                                },
                                Semantic::TexCoords(idx)=>{
                                    let count = count*2;
                                    let slice = unsafe {std::slice::from_raw_parts(p,count)};
                                    vertex_attr.tex_coord.extend([0.0].iter().cycle().take(count));
                                    vertex_attr.tex_coord.copy_from_slice(slice);
                                    println!("TexCoord total {},need {},offset {},count {}",buffer.len(),view.length(),view.offset(),count)
                                },
                                Semantic::Colors(rgb)=>{
                                    let count = count*3;
                                    let slice = unsafe {std::slice::from_raw_parts(p,count)};
                                    vertex_attr.color.extend([0.0].iter().cycle().take(count));
                                    vertex_attr.color.copy_from_slice(slice);
                                    println!("Color total {},need {},offset {},count {}",buffer.len(),view.length(),view.offset(),count)
                                }
                                _ => {}
                            }
                        }

                        if vertex_attr.color.len()==0 {
                            let count = vertex_attr.position.len();
                            vertex_attr.color.extend([1.0].iter().cycle().take(count));
                        }

                        meshes.push(Mesh::new(vertex_attr,indices))
                    }

                    let mut material = Material::new(textures,base_shader);
                    let mut transform = TMat4::default();
                    transform.fill_with_identity();
                    let model = Model::new(meshes,material,transform);
                    models.push(model)
                }
                //提取相机
                if node.name()==Some("Camera") {
                    let ([px,py,pz],[rx,ry,rz,rw],scale) = node.transform().decomposed();
                    camera.set_position(px,py,pz);
                    let rotation = quat(rx,ry,rz,rw);
                    let angle = quat_euler_angles(&rotation);

                    let yaw = angle.y*180.0/PI;
                    let pitch = angle.x*180.0/PI;
                    let roll = 0.0f32;
                    println!("camera angle {},{},{}",angle.x,angle.y*180.0/PI,angle.z*180.0/PI);

                    camera.set_rotation(yaw,pitch,roll)
                }
            }
        }

        Scene{models,camera}
    }
}
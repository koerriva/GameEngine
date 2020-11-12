use crate::engine::graph::mesh::{Mesh, VertexAttr};
use crate::engine::graph::material::Material;
use crate::engine::camera::Camera;
use nalgebra_glm::{TMat4, TVec3, rotate, radians, TVec1, scale, vec3};
use std::f32::consts::PI;
use std::path::Path;
use crate::engine::graph::light::Light;
use gltf::Semantic;
use crate::engine::graph::shader::ShaderProgram;
use gltf::json::accessor::Type;
use crate::engine::graph::texture::Texture;
use gltf::image::Format;
use std::time::SystemTime;

pub struct Model{
    meshes:Vec<Mesh>,
    material:Material,
    pub transform:TMat4<f32>
}

pub struct Scene{
    pub camera:Camera,
    pub models:Vec<Model>,
}

impl Model{
    pub fn new(meshes:Vec<Mesh>,material:Material)->Self{
        let mut transform:TMat4<f32> = TMat4::default();
        transform.fill_with_identity();
        Model{meshes,material,transform}
    }

    pub fn draw(&self,camera:&Camera){
        self.material.bind();
        self.material.set_pvm(&camera.projection_matrix(),&camera.view_matrix(),&self.transform);
        self.material.set_view_pos(&camera.position);
        self.material.set_time();
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
    pub fn add_model(&mut self,model:Model){
        self.models.push(model)
    }

    pub fn empty()->Scene{
        let mut models = Vec::new();
        let mut camera = Camera::new(4.0/3.0);
        camera.move_forward(-5.0);
        Scene{camera,models}
    }

    pub fn from_gltf(path:&str)->Scene{
        let mut models = Vec::new();
        let mut camera = Camera::new(4.0/3.0);
        camera.move_forward(-5.0);
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
                    let mut textures = Vec::new();

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

                        let mat = primitive.material();
                        let pbr_desc = mat.pbr_metallic_roughness();
                        let [r,g,b,a] = pbr_desc.base_color_factor();
                        let base_color_texture = pbr_desc.base_color_texture();
                        if base_color_texture.is_none(){
                            println!("生成漫反射贴图...");
                            let r = (r*255.99) as u8;
                            let g = (g*255.99) as u8;
                            let b = (b*255.99) as u8;
                            let a = (a*255.99) as u8;
                            let texture = Texture::new(1,1,4,&[r,g,b,a]);
                            textures.push(texture);
                        }else {
                            let gltf_texture = base_color_texture.unwrap().texture();
                            let buffer_idx = gltf_texture.index();
                            let buffer = images.get(buffer_idx).unwrap();

                            let components = match buffer.format {
                                Format::R8=> 1,
                                Format::R8G8=> 2,
                                Format::R8G8B8 => 3,
                                Format::R8G8B8A8 => 4,
                                _=>panic!("无法识别的像素格式")
                            };
                            println!("生成漫反射贴图... {},{},{:?}",buffer.width,buffer.height,buffer.format);
                            let texture = Texture::new(buffer.width as i32, buffer.height as i32, components, buffer.pixels.as_slice());
                            textures.push(texture);
                        }

                        let metallic_factor= pbr_desc.metallic_factor();
                        let roughness_factor = pbr_desc.roughness_factor();
                        let metallic_roughness_texture = pbr_desc.metallic_roughness_texture();
                        if metallic_roughness_texture.is_none(){
                            println!("生成表面粗糙度贴图...");
                            let factor = metallic_factor+roughness_factor;
                            let r = (factor*255.99) as u8;
                            let texture = Texture::new(1,1,3,&[r,r,r]);
                            textures.push(texture);
                        }else{
                            let gltf_texture = metallic_roughness_texture.unwrap().texture();
                            let buffer_idx = gltf_texture.index();
                            let buffer = images.get(buffer_idx).unwrap();

                            let components = match buffer.format {
                                Format::R8=> 1,
                                Format::R8G8=> 2,
                                Format::R8G8B8 => 3,
                                Format::R8G8B8A8 => 4,
                                _=>panic!("无法识别的像素格式")
                            };
                            println!("生成表面粗糙度贴图... {},{},{:?}",buffer.width,buffer.height,buffer.format);
                            let texture = Texture::new(buffer.width as i32, buffer.height as i32, components, buffer.pixels.as_slice());
                            textures.push(texture);
                        }

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
                    let model = Model::new(meshes,material);
                    models.push(model)
                }
            }
        }

        Scene{models,camera}
    }
}
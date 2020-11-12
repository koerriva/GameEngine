use std::mem::size_of;
use std::os::raw::c_void;
use std::process::id;
use nalgebra_glm::{vec3, TVec3, normalize, cross, fast_normalize_dot};
use crate::engine::math::vec3_sub;
use nalgebra::{DimAdd, clamp};

pub struct VertexAttr{
    pub position:Vec<f32>,
    pub normal:Vec<f32>,
    pub tex_coord:Vec<f32>,
    pub color:Vec<f32>,
}

pub struct Mesh{
    vao:u32,
    vbos:[u32;4],
    ebo:u32,
    vertex_count:usize,
    vertex_attr:VertexAttr
}

impl Mesh {
    pub fn from_heightmap(width:i32,height:i32,heightmap:&[f64])->Mesh{
        let bound_x = (-5.0f32,5.0f32);
        let bound_z = (-5.0f32,5.0f32);

        let inc_x = (bound_x.0*2.0).abs()/(width-1) as f32;
        let inc_z = (bound_z.0*2.0).abs()/(height-1) as f32;
        let mut position = Vec::new();
        let mut normal = Vec::new();
        let mut tex_coord = Vec::new();
        let mut color = Vec::new();
        let mut indices = Vec::new();

        for row in 0..height {
            for col in 0..width {
                let idx = (row*width+col) as usize;
                let x = bound_x.0 + col as f32*inc_x;
                let z = bound_z.0 + row as f32*inc_z;
                let y = heightmap[idx] as f32;
                position.push(x);
                position.push(clamp(y*5.0,-1.0,5.0));
                position.push(z);
                let tex_coord_x = 1.0 * col as f32 / width as f32;
                let tex_coord_z = 1.0 * row as f32 / height as f32;
                tex_coord.push(tex_coord_x);
                tex_coord.push(tex_coord_z);

                normal.push(0.0);
                normal.push(1.0);
                normal.push(0.0);

                let c = (y+1.0)/2.0*vec3(66.0/255.9,76.0/255.9,80.0/255.9);
                color.push(c.x);
                color.push(c.y);
                color.push(c.z);

                if col<width-1&&row<height-1 {
                    let p0 = (col,row);
                    let p1 = (col+1,row);
                    let p2 = (col,row+1);
                    let p3 = (col+1,row+1);

                    let idx = |p:(i32,i32),width:i32|p.1*width+p.0;
                    indices.push(idx(p0,width) as u16);
                    indices.push(idx(p2,width) as u16);
                    indices.push(idx(p1,width) as u16);
                    indices.push(idx(p2,width) as u16);
                    indices.push(idx(p3,width) as u16);
                    indices.push(idx(p1,width) as u16);
                }
            }
        }

        for row in 1..height-1 {
            for col in 1..width-1 {
                let idx = ((row*width+col)*3) as usize;
                let p0 = &position[idx..idx+3];
                let p1 = &position[idx-3..idx];
                let p2 = &position[idx+width as usize*3..idx+width as usize*3+3];
                let p3 = &position[idx+3..idx+3+3];
                let p4 = &position[idx-width as usize*3..idx-width as usize*3+3];

                let v1 = vec3_sub(p1,p0);
                // println!("v1 {:?}-{:?}={}",p1,p0,v1);
                let v2 = vec3_sub(p2,p0);
                // println!("v2 {:?}-{:?}={}",p2,p0,v2);
                let v3 = vec3_sub(p3,p0);
                // println!("v3 {:?}-{:?}={}",p3,p0,v3);
                let v4 = vec3_sub(p4,p0);
                // println!("v4 {:?}-{:?}={}",p4,p0,v4);

                let v12:TVec3<f32> = v1.cross(&v2).normalize();
                let v23:TVec3<f32> = v2.cross(&v3).normalize();
                let v34:TVec3<f32> = v3.cross(&v4).normalize();
                let v41:TVec3<f32> = v4.cross(&v1).normalize();

                // let v12:TVec3<f32> = normalize(&cross(&v1,&v2))*1.0;
                // let v23:TVec3<f32> = normalize(&cross(&v2,&v3))*1.0;
                // let v34:TVec3<f32> = normalize(&cross(&v3,&v4))*1.0;
                // let v41:TVec3<f32> = normalize(&cross(&v4,&v1))*1.0;

                let n = v12+v23+v34+v41;
                let n:TVec3<f32> = normalize(&n)*1.0;

                // println!("normal old({},{},{}), new({},{},{})",normal[idx+0],normal[idx+1],normal[idx+2],n.x,n.y,n.z);

                normal[idx+0] = n.x;
                normal[idx+1] = n.y;
                normal[idx+2] = n.z;
            }
        }

        let vertex_attr = VertexAttr{position,normal,tex_coord,color};
        Mesh::new(vertex_attr,indices)
    }

    pub fn new(vertex_attr:VertexAttr,indices:Vec<u16>)->Mesh{
        let mut vao=0;
        let mut vbos = [0;4];
        let mut ebo = 0;
        unsafe {
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);

            //position
            let mut vbo = 0u32;
            gl::GenBuffers(1,&mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
            let buffer_size = vertex_attr.position.len() * size_of::<f32>();
            println!("position bytes {},count {}",buffer_size,vertex_attr.position.len());
            gl::BufferData(gl::ARRAY_BUFFER,buffer_size as isize,vertex_attr.position.as_ptr() as *const c_void ,gl::STATIC_DRAW);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,0,0 as *const c_void);
            vbos[0]=vbo;

            //normal
            let mut vbo = 0u32;
            gl::GenBuffers(1,&mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
            let buffer_size = vertex_attr.normal.len() * size_of::<f32>();
            println!("normal bytes {},count {}",buffer_size,vertex_attr.normal.len());
            gl::BufferData(gl::ARRAY_BUFFER,buffer_size as isize,vertex_attr.normal.as_ptr() as *const c_void ,gl::STATIC_DRAW);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1,3,gl::FLOAT,gl::FALSE,0,0 as *const c_void);
            vbos[1]=vbo;

            //text_coord
            let mut vbo = 0u32;
            gl::GenBuffers(1,&mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
            let buffer_size = vertex_attr.tex_coord.len() * size_of::<f32>();
            println!("tex_coord bytes {},count {}",buffer_size,vertex_attr.tex_coord.len());
            gl::BufferData(gl::ARRAY_BUFFER,buffer_size as isize,vertex_attr.tex_coord.as_ptr() as *const c_void ,gl::STATIC_DRAW);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2,2,gl::FLOAT,gl::FALSE,0,0 as *const c_void);
            vbos[2]=vbo;

            //color
            let mut vbo = 0u32;
            gl::GenBuffers(1,&mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
            let buffer_size = vertex_attr.color.len() * size_of::<f32>();
            println!("color bytes {},count {}",buffer_size,vertex_attr.color.len());
            gl::BufferData(gl::ARRAY_BUFFER,buffer_size as isize,vertex_attr.color.as_ptr() as *const c_void ,gl::STATIC_DRAW);
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3,3,gl::FLOAT,gl::FALSE,0,0 as *const c_void);
            vbos[3]=vbo;

            //indices
            gl::GenBuffers(1,&mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,ebo);
            let buffer_size = indices.len()*size_of::<u16>();
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,buffer_size as isize,indices.as_ptr() as *const c_void,gl::STATIC_DRAW);

            gl::BindVertexArray(0);
        }

        Mesh{vao,vbos,ebo,vertex_count:indices.len(),vertex_attr}
    }

    pub fn draw(&self){
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES,self.vertex_count as i32,gl::UNSIGNED_SHORT,std::ptr::null());
            gl::BindVertexArray(0)
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        println!("Drop Mesh {}",self.vao)
    }
}
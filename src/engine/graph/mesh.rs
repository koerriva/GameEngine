use std::mem::size_of;
use std::os::raw::c_void;
use std::fmt::Error;

#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub struct Vertex{
    position:[f32;3],
    normal:[f32;3],
    tex_coord:[f32;2],
    color:[f32;3],
}

pub struct Mesh{
    vao:u32,
    vbo:u32,
    ebo:u32,
    vertex_count:usize
}

impl Mesh {

    pub fn from_data(vertices:&[f32],indices:&[u16])->Mesh {
        let mut vertex_list = Vec::new();
        for row in 0..vertices.len() / 11 {
            let mut start = row*11;
            let mut position:[f32;3] = [0.0;3];
            position.copy_from_slice(&vertices[start..start+3]);

            start += 3;
            let mut normal:[f32;3] = [0.0;3];
            normal.copy_from_slice(&vertices[start..start+3]);

            start += 3;
            let mut tex_coord:[f32;2] = [0.0;2];
            tex_coord.copy_from_slice(&vertices[start..start+2]);

            start += 2;
            let mut color:[f32;3] = [0.0;3];
            color.copy_from_slice(&vertices[start..start+3]);

            let vertex = Vertex{position,normal,tex_coord,color};
            println!("vertex {:?}",vertex);
            vertex_list.push(vertex)
        }
        let index_list = indices.to_vec();
        println!("indices {:?}",indices);
        Mesh::new(vertex_list, index_list)
    }

    pub fn new(vertices:Vec<Vertex>,indices:Vec<u16>)->Mesh{
        let mut vao=0;
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe {
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1,&mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
            let buffer_size = vertices.len()*size_of::<Vertex>();
            gl::BufferData(gl::ARRAY_BUFFER,buffer_size as isize,vertices.as_ptr() as *const c_void ,gl::STATIC_DRAW);

            let mut offset = 0;
            let stride = (11*size_of::<f32>()) as i32;
            //position
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,stride,offset as *const c_void);
            //normal
            offset += 3*size_of::<f32>();
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1,3,gl::FLOAT,gl::FALSE,stride,offset as *const c_void);
            //text_coord
            offset += 3*size_of::<f32>();
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2,2,gl::FLOAT,gl::FALSE,stride,offset as *const c_void);
            //color
            offset += 2*size_of::<f32>();
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3,3,gl::FLOAT,gl::FALSE,stride,offset as *const c_void);

            //indices
            gl::GenBuffers(1,&mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,ebo);
            let buffer_size = indices.len()*size_of::<u16>();
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,buffer_size as isize,indices.as_ptr() as *const c_void,gl::STATIC_DRAW);

            gl::BindVertexArray(0);
        }

        Mesh{vao,vbo,ebo,vertex_count:indices.len()}
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
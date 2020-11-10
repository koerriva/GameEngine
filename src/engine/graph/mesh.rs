use std::mem::size_of;
use std::os::raw::c_void;

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
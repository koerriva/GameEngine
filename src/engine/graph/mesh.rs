use std::mem::size_of;
use std::os::raw::c_void;
use std::process::id;

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
    pub fn from_heightmap(width:i32,height:i32,heightmap:&[f32])->Mesh{
        let bound_x = (-0.5f32,0.5f32);
        let bound_z = (-0.5f32,0.5f32);

        let inc_x = bound_x.0/(width-1) as f32;
        let inc_z = bound_z.1/(height-1) as f32;
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
                let y = heightmap[idx];
                position.push(x);
                position.push(y);
                position.push(z);
                let tex_coord_x = col as f32 / width as f32;
                let tex_coord_z = row as f32 / height as f32;
                tex_coord.push(tex_coord_x);
                tex_coord.push(tex_coord_z);

                normal.push(0.0);
                normal.push(1.0);
                normal.push(0.0);

                let color_r = tex_coord_x;
                let color_g = tex_coord_z;
                let color_b = 0.4f32;
                color.push(color_r);
                color.push(color_g);
                color.push(color_b);

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

        // for row in 0..height - 1 {
        //     for col in 0..width - 1 {
        //
        //     }
        // }

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
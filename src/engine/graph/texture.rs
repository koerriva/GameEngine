use std::ffi::c_void;
use gl::types::*;

#[derive(Debug)]
pub struct Texture{
    pub id:u32,
    pub width:i32,
    pub height:i32,
    pub components:i32
}

impl Texture {
    pub fn new(width:i32,height:i32,components:i32,data:&[u8])->Texture{
        let mut id:u32 = 0;
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT,components);
            gl::GenTextures(1,&mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            let format:u32 = match components {
                1 => gl::RED,
                2 => gl::RG,
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => panic!("无法识别的格式")
            };
            gl::TexImage2D(gl::TEXTURE_2D,0,format as i32,width,height,0,format,gl::UNSIGNED_BYTE,data.as_ptr() as *const c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D,0);
        }
        Texture{id,width,height,components}
    }

    pub fn from_font(width:i32,height:i32,data:&[u8])->Texture{
        let mut id:u32 = 0;
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT,1);
            gl::GenTextures(1,&mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(gl::TEXTURE_2D,0,gl::RED as i32,width,height,0,gl::RED,gl::UNSIGNED_BYTE,data.as_ptr() as *const c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D,0);
        }
        Texture{id,width,height,components:1}
    }

    pub fn set_wrap_mode(&self,wrap_mode:i32){
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D,self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_mode);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_mode);
            gl::BindTexture(gl::TEXTURE_2D,0);
        }
    }

    pub fn set_filter_mode(&self,filter_mode:i32){
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D,self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter_mode);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter_mode);
            gl::BindTexture(gl::TEXTURE_2D,0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        println!("Drop Texture {}",self.id)
    }
}
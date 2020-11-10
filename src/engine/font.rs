use freetype::{library, Library, Face};
use std::collections::HashMap;
use freetype::face::LoadFlag;
use std::ops::Add;
use std::path::PathBuf;
use crate::engine::graph::texture::Texture;
use std::any::Any;
use crate::engine::graph::shader::ShaderProgram;

#[derive(Debug)]
pub struct Character{
    pub texture:Option<Texture>,
    pub size:(i32,i32),
    pub bearing:(i32,i32),
    pub advance:(i32,i32),
    pub buffer:Vec<u8>
}

pub struct Font{
    face:Face,
    pub height:u32,
    pub shader:Option<ShaderProgram>,
    chars:HashMap<usize,Character>
}

impl Font{
    fn load_char(face:&Face,code:usize)->Option<Character>{
        let r = face.load_char(code,LoadFlag::RENDER);
        if r.is_ok() {
            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            let advance = glyph.advance();
            let buffer = Vec::from(bitmap.buffer());
            // println!("char {},buffer size {},except {}",code,buffer.len(),bitmap.width()*bitmap.rows());
            let char = Character{
                texture:None,
                size:(bitmap.width(),bitmap.rows()),
                bearing:(glyph.bitmap_left(),glyph.bitmap_top()),
                advance:(advance.x as i32, advance.y as i32),buffer
            };
            Some(char)
        }else {
            None
        }
    }

    /**
        https://blog.csdn.net/chivalrousli/article/details/77412329
    **/
    pub fn new(name:&str,height:u32)->Font{
        let filepath = String::from("data/font/").add(name);
        let face = Library::init().unwrap().new_face(PathBuf::from(filepath),0).unwrap();
        face.set_pixel_sizes(0,height);
        let mut chars = HashMap::new();

        //基本字母
        for code in 0x0..0xff {
            let c = Font::load_char(&face,code);
            chars.insert(code,c.unwrap());
        }
        //中日韩统一表意文字
        // for code in 0x2E80..0xFE4F{
        //
        // }
        //CJK标点符号
        for code in 0x3000..0x303F{
            let c = Font::load_char(&face,code);
            chars.insert(code,c.unwrap());
        }
        //基本中文
        for code in 0x4e00..0x9fa5{
            let c = Font::load_char(&face,code);
            chars.insert(code,c.unwrap());
        }
        //全角ASCII、全角中英文标点、半宽片假名、半宽平假名、半宽韩文字母
        for code in 0xff00..0xffef{
            let c = Font::load_char(&face,code);
            chars.insert(code,c.unwrap());
        }
        Font{face,chars,height, shader: None }
    }

    pub fn init(&mut self){
        for (code, mut c) in &mut self.chars {
            let texture = Texture::from_font(c.size.0, c.size.1, c.buffer.as_slice());
            c.texture = Some(texture)
        }
    }

    pub fn read_mut(&mut self,char:usize)->Option<&Character>{
        let r = self.chars.get(&char);
        if r.is_none() {
            let r = Font::load_char(&self.face,char);
            if r.is_some() {
                let mut r = r.unwrap();
                let texture = Texture::from_font(r.size.0,r.size.1,r.buffer.as_slice());
                r.texture = Some(texture);
                self.chars.insert(char,r);
            }
        }else {
            let r = self.chars.get_mut(&char);
            if r.is_some() {
                let mut r = r.unwrap();
                if r.texture.is_none() {
                    let texture = Texture::from_font(r.size.0,r.size.1,r.buffer.as_slice());
                    r.texture = Some(texture);
                }
            }
        }
        self.chars.get(&char)
    }

    pub fn read(&self,char:usize)->Option<&Character>{
        self.chars.get(&char)
    }
}
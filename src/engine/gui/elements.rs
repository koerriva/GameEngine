use crate::engine::renderer::Drawable;
pub trait GUI{
    fn on_click(&self);
}

pub struct GUILabel{
    text:String,
    pos:(f32,f32),
    color:(f32,f32,f32)
}

impl GUILabel {
    pub fn new(text:&str,pos:(f32,f32),color:(f32,f32,f32))->Self{
        GUILabel{text:text.to_string(),pos,color}
    }
}

impl Drawable for GUILabel {
    fn draw(&self) {
        unimplemented!()
    }
}
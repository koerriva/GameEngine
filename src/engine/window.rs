pub struct Window{
    width:u32,
    height:u32,
    aspect:f32,
    closed:bool
}

impl Window {
    pub fn new(width:u32,height:u32)->Window{
        Window{width,height,aspect:4.0f32/3.0f32,closed:false}
    }
}

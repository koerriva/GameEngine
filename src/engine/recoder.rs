use std::time::Duration;
use image::{DynamicImage, save_buffer, ColorType, ImageBuffer, ImageFormat, GenericImage, Rgba};

pub struct ScreenRecoder{
}

impl ScreenRecoder {
    pub fn new()->Self{
        ScreenRecoder{}
    }

    pub fn run(&mut self)->(){
        std::thread::spawn(move||{
            loop {
                std::thread::sleep(Duration::from_millis(10));
                // println!("recording ...")
            }
        }).thread();
    }

    pub fn record(&mut self,width:i32,height:i32,data:Vec<u8>) {
        println!("record ... ");
        let mut img = DynamicImage::new_rgb8(width as u32, height as u32);
        for h in 0..height - 1 {
            for w in 0..width - 1 {
                let idx = (h*width+w)*3;
                let p = &data[idx as usize..idx as usize+3];
                img.put_pixel(w as u32, h as u32, Rgba([p[0],p[1],p[2],0]))
            }
        }
        img.save_with_format("view.png",ImageFormat::Png).expect("save picture fail")
        // save_buffer("view.png", &data, width as u32, height as u32, ColorType::Rgb8);
    }
}
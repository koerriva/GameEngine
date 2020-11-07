use glfw::{Glfw, Context, SwapInterval, WindowEvent, Key, Action, InitHint, WindowHint};
use glfw::WindowMode::Windowed;
use std::sync::mpsc::Receiver;

pub struct Window{
    pub width:u32,
    pub height:u32,
    pub aspect:f32,
    pub closed:bool,
    pub glfw:Glfw,
    pub mouse_offset:(f32,f32,f32,f32),
    canvas:glfw::Window,
    events:Receiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(width:u32,height:u32)->Window{
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(WindowHint::ContextVersion(4,1));
        glfw.window_hint(WindowHint::DoubleBuffer(true));
        glfw.window_hint(WindowHint::Resizable(false));
        glfw.window_hint(WindowHint::Samples(Some(4)));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window,events) = glfw.create_window(width, height, "Tech Demo", Windowed)
            .expect("创建窗口失败");
        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_close_polling(true);

        glfw.with_primary_monitor_mut(|_,m|{
            let vid_mode = m.unwrap().get_video_mode().unwrap();
            let xpos = (vid_mode.width-width)/2;
            let ypos = (vid_mode.height-height)/2;
            println!("screen width {},height {}", vid_mode.width, vid_mode.height);
            window.set_pos(xpos as i32, ypos as i32);
        });

        glfw.set_swap_interval(SwapInterval::Sync(1));

        gl::load_with(|s|glfw.get_proc_address_raw(s));

        Window{width,height,aspect:4.0f32/3.0f32,closed:false,glfw,mouse_offset:(0.0,0.0,0.0,0.0),canvas:window,events }
    }

    pub fn update(&mut self){
        self.input();
        self.canvas.swap_buffers();
        self.glfw.poll_events();
    }

    fn input(&mut self){
        for (_, event) in glfw::flush_messages(&self.events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.closed = true;
                    self.canvas.set_should_close(true)
                },
                glfw::WindowEvent::Close=>{
                    self.closed = true;
                    self.canvas.set_should_close(true)
                },
                glfw::WindowEvent::CursorPos(x,y)=>{
                    let (prv_x,prv_y,x0,y0) = self.mouse_offset;
                    let x1 = x as f32 - prv_x;
                    let y1 = prv_y - y as f32;
                    let sensitivity:f32 = 0.05;
                    self.mouse_offset = (x as f32,y as f32,x1*sensitivity,y1*sensitivity);
                },
                _ => {},
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        println!("Drop Window")
    }
}

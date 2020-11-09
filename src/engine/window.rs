use glfw::{Glfw, Context, SwapInterval, WindowEvent, Key, Action, InitHint, WindowHint};
use glfw::WindowMode::Windowed;
use std::sync::mpsc::Receiver;
use glfw::ffi::glfwGetTime;

pub struct Window{
    pub width:u32,
    pub height:u32,
    pub aspect:f32,
    pub closed:bool,
    pub glfw:Glfw,
    pub mouse_offset:(f32,f32,f32,f32),
    pub frame_buffer_size:(i32,i32),
    pub canvas:glfw::Window,
    events:Receiver<(f64, WindowEvent)>,
    last_frame_time:f64,
    fps:i32,
    title:&'static str
}

impl Window {
    pub fn new(width:u32,height:u32,title:&'static str)->Window{
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(WindowHint::ContextVersion(4,1));
        glfw.window_hint(WindowHint::DoubleBuffer(true));
        glfw.window_hint(WindowHint::Resizable(true));
        glfw.window_hint(WindowHint::Samples(Some(4)));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window,events) = glfw.create_window(width, height, title, Windowed)
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
        let frame_buffer_size = window.get_framebuffer_size();
        let last_frame_time = glfw.get_time();
        let aspect = width as f32/height as f32;
        Window{width,height,aspect,closed:false,glfw
            ,mouse_offset:(0.0,0.0,0.0,0.0)
            ,frame_buffer_size
            ,canvas:window
            ,events
            ,last_frame_time
            ,fps:0
            ,title
        }
    }

    pub fn update(&mut self){
        self.input();
        self.canvas.swap_buffers();
        self.glfw.poll_events();

        let now = self.glfw.get_time();
        let elapsed = now-self.last_frame_time;
        self.last_frame_time = now;
        self.fps = (1.0/elapsed) as i32;
        self.canvas.set_title(format!("{},FPS:{}",self.title,self.fps).as_str())
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
                glfw::WindowEvent::Size(w,h)=>{
                    self.aspect =  w as f32/h as f32
                },
                glfw::WindowEvent::FramebufferSize(w,h)=>{
                    self.frame_buffer_size = (w,h);
                    unsafe {gl::Viewport(0,0,w,h)}
                }
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

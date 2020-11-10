use nalgebra_glm::{TVec3, vec3, TMat, TMat4, look_at, perspective};

pub struct Camera{
    pub position:TVec3<f32>,
    pub front:TVec3<f32>,
    pub up:TVec3<f32>,
    pub right:TVec3<f32>,
    pub pitch:f32,
    pub yaw:f32,
    pub aspect:f32,
}

impl Camera{
    pub fn new(aspect:f32)->Self{
        let position = vec3(0.0,0.0,0.0);
        let front = vec3(0.0,0.0,-1.0);
        let up = vec3(0.0,1.0,0.0);
        let right = vec3(1.0,0.0,0.0);
        let pitch = 0.0;
        let yaw = -89.0;
        Camera{position,front,up,right,pitch,yaw,aspect}
    }

    pub fn view_matrix(&self)->TMat4<f32>{
        look_at(&self.position,&self.front,&self.up)
    }

    pub fn projection_matrix(&self)->TMat4<f32>{
        perspective(self.aspect,70.0,0.01,1000.0)
    }

    pub fn set_aspect(&mut self,aspect:f32){
        self.aspect = aspect
    }

    pub fn move_forward(&mut self,factor:f32){
        self.position += factor*&self.front
    }
}
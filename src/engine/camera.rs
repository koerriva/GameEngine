use nalgebra_glm::{TVec3, vec3, TMat, TMat4, look_at, perspective};

pub struct Camera{
    pub position:TVec3<f32>,
    pub front:TVec3<f32>,
    pub up:TVec3<f32>,
    pub right:TVec3<f32>,
    pub pitch:f32,
    pub yaw:f32,
}

impl Camera{
    pub fn new()->Self{
        let position = vec3(0.0,0.0,0.0);
        let front = vec3(0.0,0.0,-1.0);
        let up = vec3(0.0,1.0,0.0);
        let right = vec3(1.0,0.0,0.0);
        let pitch = 0.0;
        let yaw = -89.0;
        Camera{position,front,up,right,pitch,yaw}
    }

    pub fn view_matrix(&self)->TMat4<f32>{
        look_at(&self.position,&self.front,&self.up)
    }

    pub fn projection_matrix(&self)->TMat4<f32>{
        perspective(1280.0/720.0,70.0,0.01,1000.0)
    }
}
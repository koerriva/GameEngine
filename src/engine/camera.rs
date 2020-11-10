use nalgebra_glm::{TVec3, vec3, TMat, TMat4, look_at, perspective, scale, cross, normalize};
use nalgebra::{DimAdd, clamp};
use std::f32::consts::PI;

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
        let dir = self.position.clone()+self.front.clone();
        look_at(&self.position,&dir,&self.up)
    }

    pub fn projection_matrix(&self)->TMat4<f32>{
        perspective(self.aspect,70.0,0.1,1000.0)
    }

    pub fn set_aspect(&mut self,aspect:f32){
        self.aspect = aspect
    }

    pub fn set_position(&mut self,x:f32,y:f32,z:f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }

    pub fn set_rotation(&mut self,yaw:f32,pitch:f32,roll:f32){
        self.yaw = yaw;
        self.pitch = pitch;

        let mut _front:TVec3<f32> = vec3(0.0,0.0,0.0);
        let r_pitch = self.pitch*PI/180.0;
        let r_yaw = self.yaw*PI/180.0;

        _front.x = r_pitch.cos() * r_yaw.cos();
        _front.y = r_pitch.sin();
        _front.z = r_pitch.cos() * r_yaw.sin();

        self.front = normalize(&_front)*1.0
    }

    pub fn move_forward(&mut self,factor:f32){
        self.position += factor * &self.front
    }

    pub fn move_right(&mut self,factor:f32){
        let a = cross(&self.front,&self.up);
        self.position += factor * normalize(&a);
    }

    pub fn rotation(&mut self,xoffset:f32,yoffset:f32){
        self.yaw -= xoffset;
        self.pitch -= yoffset;
        self.pitch = clamp(self.pitch,-89.0,89.0);

        let mut _front:TVec3<f32> = vec3(0.0,0.0,0.0);
        let r_pitch = self.pitch*PI/180.0;
        let r_yaw = self.yaw*PI/180.0;

        _front.x = r_pitch.cos() * r_yaw.cos();
        _front.y = r_pitch.sin();
        _front.z = r_pitch.cos() * r_yaw.sin();

        self.front = normalize(&_front)*1.0
    }
}
use nalgebra_glm::{TVec3, vec3};

pub fn vec3_sub(a:&[f32], b:&[f32]) ->TVec3<f32>{
    vec3(a[0]-b[0],a[1]-b[1],a[2]-b[2])
}
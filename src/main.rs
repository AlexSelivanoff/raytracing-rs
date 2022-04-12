use exr::prelude::*;
use glam::*;

struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray{
    pub fn at(&self, t:f32) -> Vec3{
        return self.origin+self.direction*t;
    } 
}

fn main() {
    println!("Hello, world!");

    let image_w: usize = 800;
    let image_h: usize = 450;

    write_rgb_file("./render/test.exr", image_w, image_h, |x, y| {
        (
            x as f32 / image_w as f32,
            y as f32 / image_h as f32,
            0.0,
    
        )
    })
    .unwrap();
}

extern crate nalgebra_glm as glm;

use glm::Vec3;

mod framebuffer;
mod bmp;
mod line;
mod polygon;

use crate::bmp::WriteBmp;
use crate::framebuffer::{Framebuffer, Color}; 
use line::Line;
use crate::polygon::Polygon;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color(0x000000));
    framebuffer.clear();

    let polygon1 = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
    ];

    // Rellenar el polígono con color amarillo
    framebuffer.set_current_color(Color(0x00FFFF)); // Color amarillo
    framebuffer.filled_polygon(&polygon1);

    // Dibujar el borde del polígono con color negro
    framebuffer.set_current_color(Color(0xFFFFFF)); // Color negro
    framebuffer.draw_polygon(&polygon1);

    framebuffer.render_buffer("out.bmp");
        
    println!("Render de polígono 1");
}

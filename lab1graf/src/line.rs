use crate::framebuffer::{Framebuffer,Color}; 

pub trait Line {
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize);
}

impl Line for Framebuffer{
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        //step en "x" "y"
        //pintar de izquierda a derecha o viceversa

        let mut err = dx - dy;
        //error

        let mut x = x1 as i32;
        let mut y = y1 as i32;


        loop {
            self.point(x as usize, y as usize); // pintar con función self.point de framebuffer
            if x == x2 as i32 && y == y2 as i32 { break; }
            let e2 = 2 * err;
            if e2 > -dy { err -= dy; x += sx; }
            if e2 < dx { err += dx; y += sy; }

        }

    }
}

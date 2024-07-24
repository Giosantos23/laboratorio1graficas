use crate::framebuffer::Framebuffer; // Importa la estructura Framebuffer del módulo framebuffer
use crate::line::Line; // Importa el trait Line del módulo line
extern crate nalgebra_glm as glm;
use glm::Vec3; 

// Define el trait Polygon con los métodos draw_polygon y filled_polygon
pub trait Polygon {
    fn draw_polygon(&mut self, points: &[Vec3]);
    fn filled_polygon(&mut self, points: &[Vec3]);
}

// Implementa el trait Polygon para la estructura Framebuffer
impl Polygon for Framebuffer {
    // Implementación del método draw_polygon
    fn draw_polygon(&mut self, points: &[Vec3]) {
        // Si hay menos de 3 puntos, no hay nada que dibujar. Triangulo es el menor
        if points.len() < 3 {
            return;
        }

        // Itera sobre cada punto en el array
        for i in 0..points.len() {
            // Obtiene el punto actual
            let x1 = points[i].x as usize;
            let y1 = points[i].y as usize;
            // Obtiene el siguiente punto, o el primer punto si estamos en el último punto
            let (x2, y2) = if i == points.len() - 1 {
                (points[0].x as usize, points[0].y as usize)
            } else {
                (points[i + 1].x as usize, points[i + 1].y as usize)
            };
            self.line(x1, y1, x2, y2);
        }
    }

    // Implementación del método filled_polygon
    fn filled_polygon(&mut self, points: &[Vec3]) {
        if points.is_empty() {
            return;
        }

        let min_y = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min) as usize;
        let max_y = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max) as usize;

        for y in min_y..=max_y {
            let mut intersections = vec![];

            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];

                if (p1.y as usize) <= y && (p2.y as usize) > y || (p2.y as usize) <= y && (p1.y as usize) > y {
                    let x = p1.x + (y as f32 - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                    intersections.push(x);
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x1 = intersections[i] as usize;
                    let x2 = intersections[i + 1] as usize;
                    for x in x1..=x2 {
                        self.point(x, y); // Método para dibujar un punto individual
                    }
                }
            }
        }
    }
}

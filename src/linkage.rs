use egui::Pos2;

use crate::point::Point;

pub(crate) struct Linkage {
    pub points: Vec<Point>,
}

impl Linkage {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add_point_c(&mut self, x: f32, y: f32) {
        self.add_point(Point::new(egui::Pos2::new(x, y)));
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn _get_angle(&self, p: usize) -> Option<f32> {
        if p == 0 {
            return None;
        }
        let pre_con = self.points[p - 1].clone();
        let p = self.points[p].clone();

        let dy = p.pos.y - pre_con.pos.y;
        let dx = p.pos.x - pre_con.pos.x;

        // return theta
        Some((dy / dx).atan().to_degrees())
    }

    pub fn set_angle_progressive(&mut self, p: usize, theta: f32) {
        // update all points postceding the indicated point
        // translate all points by the same vector calculated
        // preserve rotation by rotating by the same rotation vector

        if p == 0 {
            return;
        }
        let pre_con = self.points[p - 1].clone();

        let point = &mut self.points[p];

        let old_pos = point.clone();

        let dy = point.pos.y - pre_con.pos.y;
        let dx = point.pos.x - pre_con.pos.x;

        let length = (dx.powi(2) + dy.powi(2)).sqrt();

        let p_new_y = pre_con.pos.y + length * (theta.to_radians().sin());
        let p_new_x = pre_con.pos.x + length * (theta.to_radians().cos());

        point.pos.y = p_new_y;
        point.pos.x = p_new_x;

        let new_pos = Pos2::new(p_new_x, p_new_y);

        let translation_vector = Pos2::new(new_pos.x - old_pos.pos.x, new_pos.y - old_pos.pos.y);

        // translate all points by translation vector
        for x in &mut self.points[(p + 1)..] {
            x.pos.x += translation_vector.x;
            x.pos.y += translation_vector.y;
        }

        // preserve rotation of all points
    }
}

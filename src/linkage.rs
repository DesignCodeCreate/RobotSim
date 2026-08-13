use egui::Pos2;

use crate::point::Point;

pub(crate) struct Linkage {
    pub base_position: Point,
    pub links: Vec<Link>,
}

pub struct Link {
    pub length: f32,
    pub angle: f32,
}

impl Link {
    fn new(length: f32, angle: f32) -> Self {
        Link { length, angle }
    }
}

impl Linkage {
    pub fn new(base_position: Point) -> Self {
        Self {
            base_position,
            links: Vec::new(),
        }
    }

    pub fn add_link(&mut self, length: f32, angle: f32) {
        self.links.push(Link::new(length, angle));
    }

    pub fn set_angle(&mut self, index: usize, angle: f32) {
        self.links[index].angle = angle;
    }

    pub fn calculate_angles(&mut self, tolerance: f32, target: Point) -> Vec<f32> {
        // z = l1
        // y = l2
        // big x = l3

        let z = self.links[0].length;
        let y = self.links[1].length;
        let x = self.links[2].length;

        let mut angles = Vec::new();

        let tolerance = tolerance.to_radians();

        let m = (z - x) * tolerance.sin();
        let k = (x + z) * tolerance.cos() + y;

        let bp1 = m * target.pos.x + (k * target.pos.y);
        let bp2 = k * target.pos.x - (m * target.pos.y);

        let b = bp1.atan2(bp2);

        angles.push((b + tolerance).to_degrees());
        angles.push(b.to_degrees());
        angles.push((b - tolerance).to_degrees());

        angles
    }

    pub fn calculate_positions(&self) -> Vec<Point> {
        let mut points = vec![Point::new(Pos2::new(
            self.base_position.pos.x,
            self.base_position.pos.y,
        ))];

        let mut current_point = self.base_position.clone();
        let mut world_angle = 0.;

        for link in &self.links {
            world_angle += link.angle.to_radians();
            let p1_y = link.length * world_angle.sin() + current_point.pos.y;
            let p1_x = link.length * world_angle.cos() + current_point.pos.x;

            let p = Point::new(Pos2::new(p1_x, p1_y));

            points.push(p.clone());

            current_point.pos.y = p.pos.y;
            current_point.pos.x = p.pos.x;
        }

        points
    }

    pub fn positions_from_world_angles(&self, world_angles: Vec<f32>) -> Vec<Point> {
        let mut points = vec![self.base_position.clone()];
        let mut current_point = self.base_position.clone();

        for (link, angle) in self.links.iter().zip(world_angles.iter()) {
            let angle = angle.to_radians();

            let x = current_point.pos.x + link.length * angle.cos();
            let y = current_point.pos.y + link.length * angle.sin();

            let point = Point::new(Pos2::new(x, y));

            points.push(point.clone());
            current_point = point;
        }

        points
    }
}

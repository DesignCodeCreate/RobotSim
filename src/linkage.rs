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
}

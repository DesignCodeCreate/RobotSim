use egui::Pos2;

use crate::point::Point;

pub(crate) struct Linkage {
    pub base_position: Point,
    pub links: Vec<Link>,
}

pub struct Link {
    pub length: f32,
}

impl Link {
    fn new(length: f32) -> Self {
        Link { length }
    }
}

impl Linkage {
    pub fn new(base_position: Point) -> Self {
        Self {
            base_position,
            links: Vec::new(),
        }
    }

    pub fn add_link(&mut self, length: f32) {
        self.links.push(Link::new(length));
    }

    pub fn find_optimal_tolerance_angles(&mut self, target: Point, margin: f32, step: f32) -> Vec<Vec<f32>> {
        // Bounds for tolerance are between -100.0 and 100.0
        let mut initial_t = -100.0;
        let mut error;
        let mut angles;

        let mut solutions: Vec<Vec<f32>> = Vec::new();

        while initial_t <= 100.0 {

            angles = crate::world_to_relative(&self.calculate_angles(initial_t, &target));
            let points = self.calculate_positions(&angles);
            let effector = &points[points.len() - 1];
            error = target.difference(&effector).abs();

            if error < margin {
                solutions.push(angles);
            }

            initial_t += step;

        }

        // Return angles

        solutions
    }
    pub fn calculate_angles(&mut self, tolerance: f32, target: &Point) -> Vec<f32> {
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

    pub fn calculate_positions(&self, angles: &Vec<f32>) -> Vec<Point> {
        let mut points = vec![Point::new(Pos2::new(
            self.base_position.pos.x,
            self.base_position.pos.y,
        ))];

        let mut current_point = self.base_position.clone();
        let mut world_angle = 0.;

        for (index, link) in self.links.iter().enumerate() {
            world_angle += angles[index].to_radians();
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

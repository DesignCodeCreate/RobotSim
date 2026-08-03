use egui::{Painter, Pos2, Response};

use crate::MyApp;

impl MyApp {
    fn convert_to_screen(&self, joints: &Vec<Pos2>, resp: Response) -> Vec<Pos2> {
        fn world_to_screen(world_pos: Pos2, screen_size: [f32; 2]) -> Pos2 {
            let screen_x = (world_pos.x + 1.0) * (screen_size[0] / 2.0);
            let screen_y = (1.0 - world_pos.y) * (screen_size[1] / 2.0);
            Pos2::new(screen_x, screen_y)
        }
        let screen_size = [resp.rect.width(), resp.rect.height()];

        let screen_joints: Vec<Pos2> = joints
            .iter()
            .map(|&j| {
                let s = world_to_screen(j, screen_size);
                Pos2::new(resp.rect.min.x + s.x, resp.rect.min.y + s.y)
            })
            .collect();

        screen_joints
    }

    fn draw_points(&mut self, joints: &Vec<Pos2>, painter: &Painter) {
        // draw joint circles
        for p in joints {
            painter.circle_filled(*p, 4.0, egui::Color32::from_rgb(200, 100, 100));
        }
    }

    fn draw_lines(&self, joints: &Vec<Pos2>, painter: &Painter) {
        // draw connecting line between joints
        if joints.len() >= 2 {
            for pair in joints.windows(2) {
                painter.line_segment(
                    [pair[0], pair[1]],
                    egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
                );
            }
        }
    }

    pub fn update(&mut self, resp: Response, painter: Painter) {
        let linkage = &mut self.linkage;

        for (index, angle) in self.theta_info.iter_mut().enumerate() {
            linkage.set_angle_progressive(index + 1, *angle);
        }

        let joints = linkage.points.iter().map(|p| p.pos).collect::<Vec<_>>();
        let screen_joints = self.convert_to_screen(&joints, resp);

        self.draw_points(&screen_joints, &painter);
        self.draw_lines(&screen_joints, &painter);
    }
}

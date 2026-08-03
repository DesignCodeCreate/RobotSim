use egui::{Painter, Pos2, Response};

use crate::MyApp;

impl MyApp {
    fn convert_to_screen(&self, joints: &Vec<Pos2>, resp: Response) -> Vec<Pos2> {
        let screen_center = resp.rect.center();

        let zoom = 10.0;
        let x_offset = 80.;

        let screen_joints: Vec<Pos2> = joints
            .iter()
            .map(|&j| {
                Pos2::new(
                    (screen_center.x + j.x * zoom) - x_offset,
                    screen_center.y - j.y * zoom,
                )
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
            linkage.set_angle(index, *angle);
        }

        let joints = linkage
            .calculate_positions()
            .iter()
            .map(|p| p.pos)
            .collect::<Vec<_>>();

        let screen_joints = self.convert_to_screen(&joints, resp);

        self.draw_points(&screen_joints, &painter);
        self.draw_lines(&screen_joints, &painter);
    }
}

use eframe::egui;
use egui::{CornerRadius, Pos2, Sense, Slider};

use crate::{linkage::Linkage, point::Point};
mod app;
mod linkage;
mod point;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 800.0])
            .with_resizable(false)
            .with_title("RobotSim"),
        ..Default::default()
    };

    eframe::run_native(
        "RobotSim",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    linkage: Linkage,
    target: Pos2,
    angles: Vec<f32>,
    tolerance: f32,
    solutions: Vec<Vec<f32>>,
    solution_tracker: usize,
    error_c: f32,
    n_step: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut linkage = Linkage::new(Point::new(Pos2::new(0.0, 0.0)));

        linkage.add_link(1.);
        linkage.add_link(1.);
        linkage.add_link(1.);

        let target = Pos2::new(1., 1.);
        let angles = vec![0.0, 0.0, 0.0];

        Self {
            target,
            linkage,
            angles,
            tolerance: 20.,
            solutions: vec![vec![0.0, 0.0, 0.0]],
            solution_tracker: 0,
            error_c: 0.1,
            n_step: 0.5,
        }
    }
}

pub fn world_to_relative(world: &[f32]) -> Vec<f32> {
        fn normalize_angle(angle: f32) -> f32 {
            let mut angle = angle % 360.0;

            if angle > 180.0 {
                angle -= 360.0;
            } else if angle < -180.0 {
                angle += 360.0;
            }

            angle
        }
        let mut relative = Vec::with_capacity(world.len());

        for (i, &angle) in world.iter().enumerate() {
            if i == 0 {
                relative.push(normalize_angle(angle));
            } else {
                relative.push(normalize_angle(angle - world[i - 1]));
            }
        }

        relative
    }

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::containers::Panel::top("top_panel").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("2D linkage simulation");
                });
            });

            egui::Panel::bottom("bottom_panel").show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Bottom panel");
                });

                ui.spacing_mut().slider_width = 700.;

                for angle in self.angles.iter_mut() {
                    ui.add(Slider::new(angle, -180.0..=180.0));

                }

                ui.label("Target position");
                ui.add(Slider::new(&mut self.target.x, 0.0..=10.0));
                ui.add(Slider::new(&mut self.target.y, 0.0..=10.0));


                ui.label("Tolerance");
                ui.add(Slider::new(&mut self.tolerance, -100.0..=100.0));

                if ui.button("Calculate Angles Analytically").clicked() {
                    self.angles = world_to_relative(&self.linkage.calculate_angles(self.tolerance, &point::Point { pos: self.target }));
                }


                ui.label("\n\n\nNumerical Algorithms ");

                if ui.button("Calculate Angles Numerically").clicked() {
                    self.solution_tracker = 0;
                    self.solutions = self.linkage.find_optimal_tolerance_angles(point::Point { pos: self.target }, self.error_c, self.n_step);
                    self.angles = <std::vec::Vec<f32> as Clone>::clone(&self.solutions[self.solution_tracker]);

                }

                if ui.button("Next solution").clicked() {
                    if self.solutions.len() > self.solution_tracker + 1 {
                        self.solution_tracker += 1;
                    } else {
                        self.solution_tracker = 0;
                    }
                    self.angles = <std::vec::Vec<f32> as Clone>::clone(&self.solutions[self.solution_tracker]);
                }
                ui.label("Error capacity");
                ui.add(Slider::new(&mut self.error_c, 0.1..=1.0));

                ui.label("Solver Step");
                ui.add(Slider::new(&mut self.n_step, 0.1..=1.0));


            });

            // allocate a painter that fills the remaining central panel area
            let avail = ui.available_size();
            let (resp, painter) = ui.allocate_painter(avail, Sense::hover());

            // draw a bordered area for the painter
            painter.rect_stroke(
                resp.rect,
                CornerRadius::default(),
                egui::Stroke::new(2.0, egui::Color32::WHITE),
                egui::StrokeKind::Middle,
            );

            self.update(resp, painter);
        });
    }
}

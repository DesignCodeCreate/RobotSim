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
    theta_info: Vec<f32>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut linkage = Linkage::new(Point::new(Pos2::new(0.0, 0.0)));

        linkage.add_link(1., 0.);
        linkage.add_link(1., 0.);
        linkage.add_link(1., 0.);
        // linkage.add_link(1., 0.);

        let target = Pos2::new(1., 1.);
        let mut x: Vec<f32> = vec![0.0; linkage.links.len()];

        let angles = linkage.calculate_angles(110.566, Point::new(target));
        println!("{:?}", angles);

        Self {
            target,
            linkage,
            theta_info: angles,
        }
    }
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

                for angle in self.theta_info.iter_mut() {
                    ui.add(Slider::new(angle, -90.0..=90.0));
                }
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

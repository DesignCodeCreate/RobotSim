use egui::Pos2;

#[derive(Clone, Debug)]
pub(crate) struct Point {
    pub pos: Pos2,
}

impl Point {
    pub fn new(pos: Pos2) -> Self {
        Self { pos }
    }
}

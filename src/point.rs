use egui::Pos2;

#[derive(Clone, Debug)]
pub(crate) struct Point {
    pub pos: Pos2,
}

impl Point {
    pub fn new(pos: Pos2) -> Self {
        Self { pos }
    }

    pub fn difference(&self, other: &Point) -> f32 {
        let dx = self.pos.x - other.pos.x;
        let dy = self.pos.y - other.pos.y;

        let s = dx.powi(2) + dy.powi(2);
        s.sqrt()
    }
}

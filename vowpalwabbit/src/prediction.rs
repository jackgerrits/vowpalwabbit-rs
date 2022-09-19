#[derive(Debug)]
pub enum Prediction {
    ActionScores { values: Vec<(u32, f32)> },
}

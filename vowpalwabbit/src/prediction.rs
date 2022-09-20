#[derive(Debug)]
pub enum Prediction {
    ActionScores { values: Vec<(u32, f32)> },
    ActionProbs { values: Vec<(u32, f32)> },
}

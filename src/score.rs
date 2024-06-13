#[derive(Debug, Clone)]
pub struct Score {
    precision: f32,
    recall: f32,
    fmeasure: f32,
}

impl Score {
    pub fn new(precision: f32, recall: f32) -> Score {
        let fmeasure = if (precision + recall) > 0.0 {
            (2.0 * precision * recall) / (precision + recall)
        } else {
            0.0
        };

        Score {
            precision,
            recall,
            fmeasure,
        }
    }
}

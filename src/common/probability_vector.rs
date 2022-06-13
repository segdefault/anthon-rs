use ordered_float::OrderedFloat;

pub struct ProbabilityVector {
    probabilities: Vec<f32>,
    sensitivity: f32,
}

impl ProbabilityVector {
    pub fn new(capacity: usize, sensitivity: f32) -> Self {
        let mut probabilities = Vec::with_capacity(capacity);
        let init_val = (1f64 / capacity as f64) as f32;

        (0..capacity).for_each(|_| probabilities.push(init_val));
        ProbabilityVector {
            probabilities,
            sensitivity,
        }
    }

    pub fn probabilities(&self) -> &[f32] {
        self.probabilities.as_ref()
    }

    pub fn adjust(&mut self, index: usize) {
        let inverse = 1f32 - self.sensitivity;

        self.probabilities = self.probabilities.iter().map(|n| n * inverse).collect();
        self.probabilities[index] += self.sensitivity;

        // Adjust floating-point errors
        // let sum: f32 = self.probabilities.iter().sum();
        // self.probabilities[index] += 1f32 - sum;
        self.adjust_floating_point_errors();
    }

    pub fn rebalance(&mut self) {
        let inverse = 1f32 - self.sensitivity;
        let expected_val = (1f64 / self.probabilities.len() as f64) as f32;

        self.probabilities = self
            .probabilities
            .iter()
            .map(|val| val + (val - expected_val) * inverse)
            .collect();

        self.adjust_floating_point_errors();
    }

    fn adjust_floating_point_errors(&mut self) {
        let delta: f32 =
            (1f32 - self.probabilities.iter().sum::<f32>()) / self.probabilities.len() as f32;

        self.probabilities = self.probabilities.iter().map(|val| val + delta).collect();
    }

    pub fn max(&self) -> Option<(usize, f32)> {
        let entry = self
            .probabilities
            .iter()
            .enumerate()
            .max_by_key(|(_, val)| OrderedFloat(**val));

        if let Some((idx, val)) = entry {
            Some((idx, *val))
        } else {
            None
        }
    }
}

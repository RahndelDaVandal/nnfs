use std::{
    process,
    sync::atomic::{
        AtomicUsize,
        Ordering
    },
};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Nueron {
    pub id: usize,
    pub weights: Vec<f32>,
    pub bias: f32,
}

impl Nueron {
    pub fn new(weights: Vec<f32>, bias: f32) -> Nueron {
        let nueron = Nueron {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            weights: weights,
            bias: bias,
        };
        log::debug!("Nueron created: {:?}", nueron);
        nueron
    }
    pub fn output(&self, inputs: &Vec<f32>) -> f32 {
        // Verify inputs and weights have the same length
        if inputs.len() != self.weights.len() {
            log::error!("Nueron::compute: inputs and self.weights must have the same length.");
            process::exit(1);
        } else {
            let mut output: f32 = 0.0;
            for i in 0..self.weights.len() {
                output += inputs[i] * self.weights[i];
            }
            output + self.bias
        }
    }
}

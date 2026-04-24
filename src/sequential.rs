use slp::{forward, Sample, Weights, IMAGE_SIZE};

// Train a perceptron sequentially — one sample at a time, one thread.
// This is the ground-truth baseline everything else is compared against.
pub fn train(samples: &[Sample], epochs: usize, learning_rate: f32, show_data: bool) -> Weights {
    let mut weights = Weights::random();

    for epoch in 0..epochs {
        let mut mistakes = 0usize;

        for sample in samples {
            // Forward pass — predict the class for this image
            let predicted = forward(&sample.pixels, &weights);

            if predicted != sample.label {
                mistakes += 1;

                // Perceptron update rule:
                // Reward the correct class by nudging its weights toward this image's pixels.
                // Penalize the wrong class by nudging its weights away.
                // learning_rate controls how big each step is.
                for i in 0..IMAGE_SIZE {
                    weights.w[sample.label][i] += learning_rate * sample.pixels[i];
                    weights.w[predicted][i]    -= learning_rate * sample.pixels[i];
                }
                // Also update the bias terms — these shift the score independent of the image
                weights.b[sample.label] += learning_rate;
                weights.b[predicted]    -= learning_rate;
            }
        }
        if show_data {
        println!("epoch {}/{} — training error: {:.1}%", epoch + 1, epochs, mistakes as f64 / samples.len() as f64 * 100.0);
        }
    }

    weights
}

// Classify a slice of samples using a trained weight matrix.
// Each sample is run through the forward pass independently.
pub fn infer(samples: &[Sample], weights: &Weights) -> Vec<usize> {
    samples.iter().map(|s| forward(&s.pixels, weights)).collect()
}
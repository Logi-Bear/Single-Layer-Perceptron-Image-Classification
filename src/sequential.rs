use slp::{forward, Sample, Weights, IMAGE_SIZE};

pub fn train(samples: &[Sample], epochs: usize, learning_rate: f32) -> Weights {
    let mut weights = Weights::random();

    for epoch in 0..epochs {
        let mut mistakes = 0usize;

        for sample in samples {
            let predicted = forward(&sample.pixels, &weights);

            if predicted != sample.label {
                mistakes += 1;

                for i in 0..IMAGE_SIZE {
                    weights.w[sample.label][i]  += learning_rate * sample.pixels[i];
                    weights.w[predicted][i]     -= learning_rate * sample.pixels[i];
                }
                weights.b[sample.label] += learning_rate;
                weights.b[predicted]    -= learning_rate;
            }
        }

        println!(
            "epoch {}/{} — training error: {:.1}%",
            epoch + 1,
            epochs,
            mistakes as f64 / samples.len() as f64 * 100.0
        );
    }

    weights
}

pub fn infer(samples: &[Sample], weights: &Weights) -> Vec<usize> {
    samples.iter().map(|s| forward(&s.pixels, weights)).collect()
}
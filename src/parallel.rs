use rayon::prelude::*;
use slp::{forward, Sample, Weights, IMAGE_SIZE, NUM_CLASSES};

pub fn train(samples: &[Sample], epochs: usize, learning_rate: f32, batch_size: usize) -> Weights {
    let mut weights = Weights::random();

    for epoch in 0..epochs {
        let delta = samples
            .par_chunks(batch_size)
            .map(|batch| {
                let mut local = Weights::zeros();
                for sample in batch {
                    let predicted = forward(&sample.pixels, &weights);
                    if predicted != sample.label {
                        for i in 0..IMAGE_SIZE {
                            local.w[sample.label][i] += learning_rate * sample.pixels[i];
                            local.w[predicted][i]    -= learning_rate * sample.pixels[i];
                        }
                        local.b[sample.label] += learning_rate;
                        local.b[predicted]    -= learning_rate;
                    }
                }
                local
            })
            .reduce(Weights::zeros, |mut acc, d| {
                for c in 0..NUM_CLASSES {
                    for i in 0..IMAGE_SIZE {
                        acc.w[c][i] += d.w[c][i];
                    }
                    acc.b[c] += d.b[c];
                }
                acc
            });

        for c in 0..NUM_CLASSES {
            for i in 0..IMAGE_SIZE {
                weights.w[c][i] += delta.w[c][i];
            }
            weights.b[c] += delta.b[c];
        }

        println!("epoch {}/{} complete", epoch + 1, epochs);
    }

    weights
}

pub fn infer(samples: &[Sample], weights: &Weights) -> Vec<usize> {
    samples.par_iter().map(|s| forward(&s.pixels, weights)).collect()
}
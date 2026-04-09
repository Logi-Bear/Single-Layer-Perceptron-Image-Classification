use rayon::prelude::*;
use slp::{forward, Sample, Weights, IMAGE_SIZE, NUM_CLASSES};

// Train using Rayon's parallel iterators.
//
// Strategy: divide samples into chunks, process all chunks in parallel,
// then aggregate the resulting deltas and apply them once per epoch.
// No locks needed — threads never share mutable state during computation.
pub fn train(samples: &[Sample], epochs: usize, learning_rate: f32, batch_size: usize) -> Weights {
    let mut weights = Weights::random();

    for epoch in 0..epochs {
        // par_chunks splits the slice into chunks of batch_size and processes them in parallel across Rayon's thread pool.
        // Each chunk produces a local Weights delta — no shared state here.
        let delta = samples.par_chunks(batch_size).map(|batch| {
            // Each thread accumulates its own local delta — no locking needed
            let mut local = Weights::zeros();
            for sample in batch {
                let predicted = forward(&sample.pixels, &weights);
                if predicted != sample.label {
                    // Same update rule as sequential, but into local delta instead of weights
                    for i in 0..IMAGE_SIZE {
                        local.w[sample.label][i] += learning_rate * sample.pixels[i];
                        local.w[predicted][i]    -= learning_rate * sample.pixels[i];
                    }
                    local.b[sample.label] += learning_rate;
                    local.b[predicted]    -= learning_rate;
                }
            }
            local
        }).reduce(Weights::zeros, |mut acc, d| {    // .reduce() folds all the per-chunk deltas into one combined delta. Weights::zeros is the identity value (like 0 in addition).
            for c in 0..NUM_CLASSES {               // This runs after all threads finish — it's sequential but very fast.
                for i in 0..IMAGE_SIZE {
                    acc.w[c][i] += d.w[c][i];
                }
                acc.b[c] += d.b[c];
            }
            acc
        });

        // Apply the aggregated delta to the real weights once per epoch
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

// Classify samples in parallel — each sample is independent so this is trivial.
// par_iter() automatically distributes work across Rayon's thread pool.
pub fn infer(samples: &[Sample], weights: &Weights) -> Vec<usize> {
    samples.par_iter().map(|s| forward(&s.pixels, weights)).collect()
}
use std::sync::{Arc, RwLock};
use std::thread;

use slp::{forward, Sample, Weights, IMAGE_SIZE, NUM_CLASSES};

// Train using manual threads sharing weights through Arc<RwLock<Weights>>.
//
// Strategy: threads share the real weight matrix.
// Each thread reads a snapshot, computes its delta locally, then writes
// the delta back. The write lock serializes the update step — only one
// thread can write at a time. This causes more contention than Rayon
// but updates the live weights more frequently, which helps accuracy.
pub fn train(samples: &[Sample], epochs: usize, learning_rate: f32, num_threads: usize, batch_size: usize, show_data: bool) -> Weights {
    // Wrap weights in Arc<RwLock> so they can be safely shared across threads.
    // Arc = shared ownership (reference counted).
    // RwLock = many readers allowed, only one writer at a time.
    let weights = Arc::new(RwLock::new(Weights::random()));

    for epoch in 0..epochs {
        // Split training data into owned chunks — threads need owned data
        // because borrowed data can't be sent across thread boundaries in Rust
        let mut mistakes = 0usize;
        let chunks: Vec<Vec<Sample>> = samples.chunks(batch_size).map(|c| c.to_vec()).collect();

        // Process num_threads chunks at a time
        for group in chunks.chunks(num_threads) {
            let handles: Vec<_> = group.iter().map(|batch| {
                let batch = batch.clone();
                // Arc::clone doesn't copy the weights — it just increments the
                // reference count and gives this thread another pointer to the same data
                let weights = Arc::clone(&weights);
                let lr = learning_rate;

                // Spawn a new OS thread. `move` transfers ownership of batch,
                // weights, and lr into the thread's closure.
                thread::spawn(move || {
                    // Step 1: acquire a read lock and clone the current weights.
                    // Multiple threads can hold the read lock simultaneously.
                    // We clone immediately so we only hold the lock briefly.
                    let snapshot = weights.read().unwrap().clone();

                    // Step 2: compute our delta using the snapshot — no lock held here.
                    // This is the bulk of the work and runs fully in parallel.
                    let mut delta = Weights::zeros();
                    let mut local_mistakes = 0usize;
                    for sample in &batch {
                        let predicted = forward(&sample.pixels, &snapshot);
                        if predicted != sample.label {
                            local_mistakes += 1;
                            for i in 0..IMAGE_SIZE {
                                delta.w[sample.label][i] += lr * sample.pixels[i];
                                delta.w[predicted][i]    -= lr * sample.pixels[i];
                            }
                            delta.b[sample.label] += lr;
                            delta.b[predicted]    -= lr;
                        }
                    }

                    // Step 3: acquire the write lock and apply our delta.
                    // Only one thread can be here at a time — others wait.
                    // This is where synchronization overhead comes from.
                    // The lock is released automatically when `w` goes out of scope.
                    let mut w = weights.write().unwrap();
                    for c in 0..NUM_CLASSES {
                        for i in 0..IMAGE_SIZE {
                            w.w[c][i] += delta.w[c][i];
                        }
                        w.b[c] += delta.b[c];
                    }
                    local_mistakes
                })
            }).collect();

            // Wait for all threads in this group to finish before starting the next group
            for h in handles {
                mistakes += h.join().unwrap();
            }
        }
        if show_data{
            println!("epoch {}/{} — training error: {:.1}%", epoch + 1, epochs, mistakes as f64 / samples.len() as f64 * 100.0);
        }
    }

    // Training is done — unwrap the Arc (we're the only owner now) to get the
    // RwLock, then call into_inner() to get the Weights out of the lock.
    Arc::try_unwrap(weights).unwrap().into_inner().unwrap()
}
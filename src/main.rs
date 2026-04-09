mod sequential;
mod parallel_rayon;
mod parallel_rwlock;

use std::time::Instant;
use slp::{accuracy, load_mnist};

fn main() {
    println!("Loading MNIST...");
    let (train, test) = load_mnist(60_000, 10_000);
    println!("  {} train, {} test samples", train.len(), test.len());

    // Extract just the labels from the test set for accuracy comparison later
    let labels: Vec<usize> = test.iter().map(|s| s.label).collect();

    // Sequential
    // Single-threaded baseline — one sample at a time, weights updated immediately
    println!("\n[Sequential]");
    let t = Instant::now();
    let weights = sequential::train(&train, 10, 0.01);
    let train_seq = t.elapsed().as_secs_f64();

    let t = Instant::now();
    let preds = sequential::infer(&test, &weights);
    let infer_seq = t.elapsed().as_secs_f64();

    println!("  Train: {:.2}s  Infer: {:.2}s  Accuracy: {:.1}%",
        train_seq, infer_seq, accuracy(&preds, &labels));

    // Rayon
    // Splits data into chunks, each chunk processed by a separate thread.
    // Deltas are computed independently then aggregated — no locks needed.
    println!("\n[Parallel — Rayon, batch_size=64]");
    let t = Instant::now();
    let weights = parallel_rayon::train(&train, 20, 0.01, 64);
    let train_rayon = t.elapsed().as_secs_f64();

    let t = Instant::now();
    let preds = parallel_rayon::infer(&test, &weights);
    let infer_rayon = t.elapsed().as_secs_f64();

    println!("  Train: {:.2}s  Infer: {:.2}s  Accuracy: {:.1}%",
        train_rayon, infer_rayon, accuracy(&preds, &labels));

    // RwLock
    // Threads share the real weight matrix via Arc<RwLock>.
    // Each thread reads weights, computes a delta, then writes the delta back.
    // The write lock serializes updates — only one thread can write at a time.
    println!("\n[Parallel — RwLock, 12 threads, batch_size=128]");
    let t = Instant::now();
    let weights = parallel_rwlock::train(&train, 10, 0.01, 12, 128);
    let train_rwlock = t.elapsed().as_secs_f64();

    let t = Instant::now();
    let preds = sequential::infer(&test, &weights);
    let infer_rwlock = t.elapsed().as_secs_f64();

    println!("  Train: {:.2}s  Infer: {:.2}s  Accuracy: {:.1}%",
        train_rwlock, infer_rwlock, accuracy(&preds, &labels));

    // Speedup
    // Speedup = sequential_time / parallel_time
    // A speedup of 2x means the parallel version finished in half the time
    println!("\n[Speedup vs Sequential]");
    println!("  Rayon  — Train: {:.2}x  Infer: {:.2}x",
        train_seq / train_rayon, infer_seq / infer_rayon);
    println!("  RwLock — Train: {:.2}x  Infer: {:.2}x",
        train_seq / train_rwlock, infer_seq / infer_rwlock);
}
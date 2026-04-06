mod sequential;
mod parallel;

use std::time::Instant;
use slp::{accuracy, load_mnist};

fn main() {
    println!("Loading MNIST...");
    let (train, test) = load_mnist(60_000, 10_000);
    println!("  {} train, {} test samples", train.len(), test.len());

    let labels: Vec<usize> = test.iter().map(|s| s.label).collect();

     // --- Sequential ---
    println!("\n[Sequential]");
    let t = Instant::now();
    let weights = sequential::train(&train, 10, 0.01);
    let train_seq = t.elapsed().as_secs_f64();
 
    let t = Instant::now();
    let preds = sequential::infer(&test, &weights);
    let infer_seq = t.elapsed().as_secs_f64();
 
    println!("  Train: {:.2}s  Infer: {:.2}s  Accuracy: {:.1}%",
        train_seq, infer_seq, accuracy(&preds, &labels));
 
    // --- Parallel (Rayon) ---
    println!("\n[Parallel — Rayon, batch_size=512]");
    let t = Instant::now();
    let weights = parallel::train(&train, 20, 0.001, 512);
    let train_par = t.elapsed().as_secs_f64();
 
    let t = Instant::now();
    let preds = parallel::infer(&test, &weights);
    let infer_par = t.elapsed().as_secs_f64();
 
    println!("  Train: {:.2}s  Infer: {:.2}s  Accuracy: {:.1}%",
        train_par, infer_par, accuracy(&preds, &labels));
 
    // --- Speedup ---
    println!("\n[Speedup]");
    println!("  Train: {:.2}x", train_seq / train_par);
    println!("  Infer: {:.2}x", infer_seq / infer_par);
}
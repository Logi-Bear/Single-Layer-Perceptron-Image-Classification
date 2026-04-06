mod sequential;

use slp::{accuracy, load_mnist};

fn main() {
    println!("Loading MNIST...");
    let (train, test) = load_mnist(60_000, 10_000);
    println!("  {} train, {} test samples", train.len(), test.len());

    println!("Training...");
    let weights = sequential::train(&train, 10, 0.01);

    println!("Classifying...");
    let predictions = sequential::infer(&test, &weights);
    let labels: Vec<usize> = test.iter().map(|s| s.label).collect();

    println!("Accuracy: {:.1}%", accuracy(&predictions, &labels));
}
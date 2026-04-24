use slp::{accuracy, forward, load_mnist, Weights, IMAGE_SIZE, NUM_CLASSES};

#[test]
fn zeros_are_all_zero() {
    let w = Weights::zeros();
    assert!(w.w.iter().all(|row| row.iter().all(|&v| v == 0.0)));
    assert!(w.b.iter().all(|&v| v == 0.0));
}

#[test]
fn random_weights_have_correct_shape() {
    let w = Weights::random();
    assert_eq!(w.w.len(), NUM_CLASSES);
    assert!(w.w.iter().all(|row| row.len() == IMAGE_SIZE));
    assert_eq!(w.b.len(), NUM_CLASSES);
}

#[test]
fn forward_returns_valid_class() {
    let w = Weights::random();
    let pixels = vec![0.5f32; IMAGE_SIZE];
    let pred = forward(&pixels, &w);
    assert!(pred < NUM_CLASSES);
}

#[test]
fn forward_is_deterministic() {
    let w = Weights::random();
    let pixels = vec![0.5f32; IMAGE_SIZE];
    assert_eq!(forward(&pixels, &w), forward(&pixels, &w));
}

#[test]
fn forward_with_zero_weights_returns_a_valid_class() {
    // All scores are 0.0 — any class is a valid argmax
    let w = Weights::zeros();
    let pixels = vec![0.5f32; IMAGE_SIZE];
    let pred = forward(&pixels, &w);
    assert!(pred < NUM_CLASSES);
}

#[test]
fn forward_responds_to_bias() {
    let mut w = Weights::zeros();
    // Push class 7 above all others via bias alone
    w.b[7] = 1.0;
    let pixels = vec![0.0f32; IMAGE_SIZE];
    assert_eq!(forward(&pixels, &w), 7);
}

#[test]
fn accuracy_perfect() {
    assert_eq!(accuracy(&[0, 1, 2, 3], &[0, 1, 2, 3]), 100.0);
}

#[test]
fn accuracy_zero() {
    assert_eq!(accuracy(&[1, 2, 3, 4], &[0, 1, 2, 3]), 0.0);
}

#[test]
fn accuracy_half() {
    let acc = accuracy(&[0, 1, 9, 9], &[0, 1, 2, 3]);
    assert!((acc - 50.0).abs() < 1e-6);
}

#[test]
fn mnist_loads_correct_counts() {
    let (train, test) = load_mnist(500, 100);
    assert_eq!(train.len(), 500);
    assert_eq!(test.len(), 100);
}

#[test]
fn mnist_pixels_are_normalized() {
    let (train, _) = load_mnist(100, 0);
    for s in &train {
        assert!(s.pixels.iter().all(|&p| p >= 0.0 && p <= 1.0));
        assert_eq!(s.pixels.len(), IMAGE_SIZE);
    }
}

#[test]
fn mnist_labels_are_valid_digits() {
    let (train, _) = load_mnist(100, 0);
    assert!(train.iter().all(|s| s.label < NUM_CLASSES));
}

#[test]
fn trained_model_beats_chance() {
    // Chance for 10 classes = 10%. Even 5 epochs on 500 samples should beat that.
    let (train, test) = load_mnist(500, 200);
    let labels: Vec<usize> = test.iter().map(|s| s.label).collect();

    let mut w = Weights::random();
    for _ in 0..5 {
        for s in &train {
            let pred = forward(&s.pixels, &w);
            if pred != s.label {
                for i in 0..IMAGE_SIZE {
                    w.w[s.label][i] += 0.01 * s.pixels[i];
                    w.w[pred][i]    -= 0.01 * s.pixels[i];
                }
                w.b[s.label] += 0.01;
                w.b[pred]    -= 0.01;
            }
        }
    }

    let preds: Vec<usize> = test.iter().map(|s| forward(&s.pixels, &w)).collect();
    assert!(
        accuracy(&preds, &labels) > 50.0,
        "expected >50% after 5 epochs, got {:.1}%", accuracy(&preds, &labels)
    );
}
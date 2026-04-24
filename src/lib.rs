use rand::Rng;

// The number of digit classes (0-9)
pub const NUM_CLASSES: usize = 10;

// 28x28 pixels flattened into a single vector
pub const IMAGE_SIZE: usize = 784;

// A single image represented as a flat vector of pixel values in [0.0, 1.0]
pub type Image = Vec<f32>;

// A training sample — one image paired with its correct label
#[derive(Clone)]
pub struct Sample {
    pub pixels: Image,
    pub label: usize,
}

// The model — a weight matrix and a bias term per class.
// w[c][i] = how strongly pixel i votes for class c
// b[c]    = a baseline score for class c independent of the image
#[derive(Clone, Debug)]
pub struct Weights {
    pub w: Vec<Vec<f32>>,  // [NUM_CLASSES][IMAGE_SIZE]
    pub b: Vec<f32>,       // [NUM_CLASSES]
}

impl Weights {
    // Initialize with small random values near zero so no class starts with an advantage
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            w: (0..NUM_CLASSES).map(|_| (0..IMAGE_SIZE).map(|_| rng.gen_range(-0.01..0.01)).collect()).collect(),
            b: vec![0.0; NUM_CLASSES],
        }
    }

    // All-zero weights — used as a starting point when accumulating gradient deltas
    pub fn zeros() -> Self {
        Self {
            w: vec![vec![0.0; IMAGE_SIZE]; NUM_CLASSES],
            b: vec![0.0; NUM_CLASSES],
        }
    }
}

// Run one image through the model and return the predicted class.
// For each class, computes: score = dot(weights[c], pixels) + bias[c]
// Returns the class with the highest score.
#[inline]
pub fn forward(pixels: &[f32], weights: &Weights) -> usize {
    (0..NUM_CLASSES).map(|c| {
        let score: f32 = weights.w[c].iter().zip(pixels.iter()).map(|(w, x)| w * x).sum::<f32>() + weights.b[c];
        // .zip(pixels.iter()) pair each weight with its corresponding pixel
        // .map(|(w, x)| w * x) multiply weight by pixel value
        // .sum::<f32>() sum all products — this is the dot product
        // + weights.b[c]; add the bias term
        (c, score)
    }).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0 // Pick the class with the highest score. Partial_cmp is needed because f32 doesn't implement total ordering (NaN edge case) and .0 returns just the class index, not the score
}

// Load MNIST from the data/ directory and return normalised train/test samples.
// max_train and max_test let you load a subset for faster dev runs.
pub fn load_mnist(max_train: usize, max_test: usize) -> (Vec<Sample>, Vec<Sample>) {
    use mnist::{Mnist, MnistBuilder};

    // The mnist crate parses the IDX binary format for us
    let Mnist { trn_img, trn_lbl, tst_img, tst_lbl, .. } = MnistBuilder::new().label_format_digit().training_set_length(60_000).test_set_length(10_000).base_path("data/").finalize();
    (to_samples(trn_img, trn_lbl, max_train), to_samples(tst_img, tst_lbl, max_test))
    
}

// Convert raw bytes into Sample structs.
// chunks(IMAGE_SIZE) splits the flat byte array into one slice per image.
// zip pairs each image slice with its label.
pub fn to_samples(images: Vec<u8>, labels: Vec<u8>, limit: usize) -> Vec<Sample> {
        images.chunks(IMAGE_SIZE).zip(labels.iter()).take(limit).map(|(chunk, &lbl)| Sample {
            // Normalize pixel values from [0, 255] to [0.0, 1.0]
            pixels: chunk.iter().map(|&p| p as f32 / 255.0).collect(),
            label: lbl as usize,
        }).collect()
}

// Compute what percentage of predictions match the true labels
pub fn accuracy(predictions: &[usize], labels: &[usize]) -> f64 {
    let correct = predictions.iter().zip(labels).filter(|(p, l)| p == l).count();
    correct as f64 / labels.len() as f64 * 100.0
}
use rand::Rng;

pub const NUM_CLASSES: usize = 10;
pub const IMAGE_SIZE: usize = 784;

pub type Image = Vec<f32>;

#[derive(Clone)]
pub struct Sample {
    pub pixels: Image,
    pub label: usize,
}

#[derive(Clone)]
pub struct Weights {
    pub w: Vec<Vec<f32>>,  // [NUM_CLASSES][IMAGE_SIZE]
    pub b: Vec<f32>,       // [NUM_CLASSES]
}

impl Weights {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            w: (0..NUM_CLASSES)
                .map(|_| (0..IMAGE_SIZE).map(|_| rng.gen_range(-0.01..0.01)).collect())
                .collect(),
            b: vec![0.0; NUM_CLASSES],
        }
    }

    pub fn zeros() -> Self {
        Self {
            w: vec![vec![0.0; IMAGE_SIZE]; NUM_CLASSES],
            b: vec![0.0; NUM_CLASSES],
        }
    }
}

#[inline]
pub fn forward(pixels: &[f32], weights: &Weights) -> usize {
    (0..NUM_CLASSES)
        .map(|c| {
            let score: f32 = weights.w[c]
                .iter()
                .zip(pixels.iter())
                .map(|(w, x)| w * x)
                .sum::<f32>()
                + weights.b[c];
            (c, score)
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
        .0
}

pub fn load_mnist(max_train: usize, max_test: usize) -> (Vec<Sample>, Vec<Sample>) {
    use mnist::{Mnist, MnistBuilder};

    let Mnist { trn_img, trn_lbl, tst_img, tst_lbl, .. } = MnistBuilder::new()
        .label_format_digit()
        .training_set_length(60_000)
        .test_set_length(10_000)
        .base_path("data/")
        .finalize();

    let to_samples = |images: Vec<u8>, labels: Vec<u8>, limit: usize| -> Vec<Sample> {
        images
            .chunks(IMAGE_SIZE)
            .zip(labels.iter())
            .take(limit)
            .map(|(chunk, &lbl)| Sample {
                pixels: chunk.iter().map(|&p| p as f32 / 255.0).collect(),
                label: lbl as usize,
            })
            .collect()
    };

    (
        to_samples(trn_img, trn_lbl, max_train),
        to_samples(tst_img, tst_lbl, max_test),
    )
}

pub fn accuracy(predictions: &[usize], labels: &[usize]) -> f64 {
    let correct = predictions.iter().zip(labels).filter(|(p, l)| p == l).count();
    correct as f64 / labels.len() as f64 * 100.0
}
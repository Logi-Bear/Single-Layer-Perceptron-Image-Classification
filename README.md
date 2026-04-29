# Single-Layer Perceptron Image Classification
**Author:** Logan McCandless  
**Course:** CSI-380 Emerging Languages  

---

## Project Description
This project implements a single-layer perceptron trained on the MNIST handwritten digit dataset.
It compares three implementations — sequential, Rayon-based parallel, and Arc/RwLock-based parallel —
measuring the performance tradeoffs between concurrency models in Rust.

- **Algorithm:** Single-Layer Perceptron
- **Dataset:** MNIST (60,000 train / 10,000 test)
- **Metrics:** Execution time, speedup, efficiency, accuracy, throughput, overhead

---

## Prerequisites
- Rust 1.75.0 or later
- Recommended: 8+ GB RAM, 4+ CPU cores
- Internet connection (for downloading the dataset)

---

## Setup Instructions

### 1. Clone the repository
```bash
git clone <your-repo-url>
cd <your-project-folder>
```

### 2. Download the MNIST dataset
Download the four MNIST binary files from http://yann.lecun.com/exdb/mnist/ and place them in a `data/` folder in the project root:
data/
├── train-images-idx3-ubyte
├── train-labels-idx1-ubyte
├── t10k-images-idx3-ubyte
└── t10k-labels-idx1-ubyte

### 3. Build and run
```bash
cargo run --release
```

### 4. Run tests
```bash
cargo test
```

---

## Project Structure
Single-Layer-Perceptron-Image-Classification/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── src/
│   ├── main.rs               # Benchmarking and entry point
│   ├── lib.rs                # Shared types and functions
│   ├── sequential.rs         # Sequential training and inference
│   ├── parallel_rayon.rs     # Parallel training using Rayon
│   └── parallel_rwlock.rs    # Parallel training using Arc/RwLock
├── data/                     # MNIST dataset files (not included)
├── tests/
│   └── integration_test.rs   # Integration tests
└── benchmarks/
└── results.md            # Benchmark results for both devices

---

## References
- Rayon documentation: https://docs.rs/rayon
- MNIST crate: https://docs.rs/mnist
- Rust standard library — `std::sync`: https://doc.rust-lang.org/std/sync/
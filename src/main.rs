mod sequential;
mod parallel_rayon;
mod parallel_rwlock;

use std::io::Write;
use std::time::Instant;
use slp::{accuracy, load_mnist};
use slp::Sample;
use slp::Weights;

struct BenchResult {
    label: String,
    num_threads: usize,
    batch_size: usize,
    train_secs: f64,
    infer_secs: f64,
    accuracy: f64,
    train_throughput: f64,
    infer_throughput: f64,
    overhead_secs: f64,
}

fn add_benchmark(acc: &mut BenchResult, b: BenchResult) {
    acc.train_secs += b.train_secs;
    acc.infer_secs += b.infer_secs;
    acc.accuracy += b.accuracy;
    acc.train_throughput += b.train_throughput;
    acc.infer_throughput += b.infer_throughput;
}

fn average_benchmark(b: &mut BenchResult, n: f64) {
    b.train_secs /= n;
    b.infer_secs /= n;
    b.accuracy /= n;
    b.train_throughput /= n;
    b.infer_throughput /= n;
}

fn run_once<TrainFn, InferFn>(
    label: &str,
    num_threads: usize,
    batch_size: usize,
    train: &[Sample],
    test: &[Sample],
    labels: &[usize],
    epochs: usize,
    train_fn: TrainFn,
    infer_fn: InferFn,
) -> BenchResult
where
    TrainFn: Fn() -> Weights,
    InferFn: Fn(&Weights) -> Vec<usize>,
{
    let t = Instant::now();
    let weights = train_fn();
    let train_secs = t.elapsed().as_secs_f64();

    let t = Instant::now();
    let preds = infer_fn(&weights);
    let infer_secs = t.elapsed().as_secs_f64();

    let acc = accuracy(&preds, labels);
    let n_train = train.len() as f64;
    let n_test = test.len() as f64;

    BenchResult {
        label: label.to_string(),
        num_threads,
        batch_size,
        train_secs,
        infer_secs,
        accuracy: acc,
        train_throughput: n_train * epochs as f64 / train_secs,
        infer_throughput: n_test / infer_secs,
        overhead_secs: 0.0,
    }
}

fn averaged_benchmark<TrainFn, InferFn>(
    label: &str,
    num_threads: usize,
    batch_size: usize,
    train: &[Sample],
    test: &[Sample],
    labels: &[usize],
    epochs: usize,
    attempts: usize,
    train_fn: TrainFn,
    infer_fn: InferFn,
) -> BenchResult
where
    TrainFn: Fn() -> Weights + Copy,
    InferFn: Fn(&Weights) -> Vec<usize> + Copy,
{
    let mut acc = run_once(label, num_threads, batch_size, train, test, labels, epochs, train_fn, infer_fn);
    for i in 1..attempts {
        print!("\r  {label}: attempt {}/{attempts}…   ", i + 1);
        std::io::stdout().flush().unwrap();
        let r = run_once(label, num_threads, batch_size, train, test, labels, epochs, train_fn, infer_fn);
        add_benchmark(&mut acc, r);
    }
    println!();
    average_benchmark(&mut acc, attempts as f64);
    println!(
        "[{label}] avg over {attempts} runs — \
         train {:.2}s  infer {:.2}ms  acc {:.2}%\n",
        acc.train_secs, acc.infer_secs * 1000.0, acc.accuracy,
    );
    acc
}

fn print_main_table(results: &mut Vec<BenchResult>) {
    let seq_train = results[0].train_secs;

    for r in results.iter_mut() {
        r.overhead_secs = if r.num_threads > 1 {
            r.train_secs - (seq_train / r.num_threads as f64)
        } else {
            0.0
        };
    }

    println!(
        "\n{:<22} {:>7}  {:>7}  {:>8}  {:>9}  {:>9}  {:>9}  {:>10}  {:>10}  {:>12}",
        "Method", "Threads", "Batch", "Train", "Infer", "Accuracy",
        "Speedup", "Efficiency", "Overhead", "Train img/s"
    );
    println!("{}", "─".repeat(125));

    for r in results.iter() {
        let speedup    = seq_train / r.train_secs;
        let efficiency = speedup   / r.num_threads as f64;
        let batch_str = if r.batch_size == 0 { "N/A".to_string() } else { r.batch_size.to_string() };
        println!(
            "{:<22} {:>7}  {:>7}  {:>8}  {:>9}  {:>9}  {:>9}  {:>10}  {:>10}  {:>12}",
            format!("{}:", r.label),
            r.num_threads,
            batch_str,
            format!("{:.2}s", r.train_secs),
            format!("{:.2}ms", r.infer_secs * 1000.0),
            format!("{:.2}%", r.accuracy),
            format!("{:.2}x", speedup),
            format!("{:.1}%", efficiency * 100.0),
            format!("{:.3}s", r.overhead_secs),
            format!("{:.0}", r.train_throughput),
        );
    }
}

fn print_thread_scalability_table(results: &[BenchResult], indices: &[usize], title: &str) {
    let seq_train = results[0].train_secs;

    println!("\n── Thread Size Scalability: {title} ──");
    println!(
        "{:>9}  {:>7}  {:>9}  {:>10}  {:>12}  {:>10}  {:>13}",
        "Threads", "Train", "Speedup", "Overhead", "Efficiency", "Accuracy", "Train img/s"
    );
    println!("{}", "─".repeat(70));

    for &idx in indices {
        let r = &results[idx];
        let speedup = seq_train / r.train_secs;
        let efficiency = speedup / r.num_threads as f64;
        println!(
            "{:>9}  {:>7}  {:>9}  {:>10}  {:>12}  {:>10}  {:>13}",
            r.num_threads,
            format!("{:.2}s", r.train_secs),
            format!("{:.2}x", speedup),
            format!("{:.3}s", r.overhead_secs),
            format!("{:.1}%", efficiency * 100.0),
            format!("{:.2}%", r.accuracy),
            format!("{:.0}", r.train_throughput),
        );
    }
}

fn print_batch_scalability_table(results: &[BenchResult], indices: &[usize], title: &str) {
    let seq_train = results[0].train_secs;

    println!("\n── Batch Size Scalability: {title} ──");
    println!(
        "{:>9}  {:>7}  {:>9}  {:>10}  {:>12}  {:>10}  {:>13}",
        "Batch", "Train", "Speedup", "Overhead", "Efficiency", "Accuracy", "Train img/s"
    );
    println!("{}", "─".repeat(70));

    for &idx in indices {
        let r = &results[idx];
        let speedup = seq_train / r.train_secs;
        let efficiency = speedup / r.num_threads as f64;
        println!(
            "{:>9}  {:>7}  {:>9}  {:>10}  {:>12}  {:>10}  {:>13}",
            r.batch_size,
            format!("{:.2}s", r.train_secs),
            format!("{:.2}x", speedup),
            format!("{:.3}s", r.overhead_secs),
            format!("{:.1}%", efficiency * 100.0),
            format!("{:.2}%", r.accuracy),
            format!("{:.0}", r.train_throughput),
        );
    }
}

fn main() {
    println!("Loading MNIST...");
    let (train, test) = load_mnist(60000, 10000);
    println!("{} train, {} test samples", train.len(), test.len());

    let labels: Vec<usize> = test.iter().map(|s| s.label).collect();
    let epochs = 15;
    let lr = 0.01f32;
    let show_epochs = false;
    let attempts = 5;
    let batch = 128;

    let mut results: Vec<BenchResult> = Vec::new();

    // 1. Sequential baseline — must stay at index 0
    results.push(averaged_benchmark(
        "Sequential", 1, 0,  // batch_size=0 signals N/A
        &train, &test, &labels, epochs, attempts,
        || sequential::train(&train, epochs, lr, show_epochs),
        |w| sequential::infer(&test, w),
    ));

    // 2. Rayon batch sweep
    let rayon_start = results.len();
    for &b in &[32_usize, 128, 512] {
        let rayon_threads = rayon::current_num_threads();
        results.push(averaged_benchmark(
            &format!("Rayon b={b}"), rayon_threads, b,
            &train, &test, &labels, epochs, attempts,
            || parallel_rayon::train(&train, epochs, lr, b, show_epochs),
            |w| parallel_rayon::infer(&test, w),
        ));
    }
    let rayon_end = results.len();

    // 3. RwLock scalability sweep: 1, 2, 4, 8, 12, 16, 20, 24 threads
    let rwlock_start = results.len();
    for &t in &[1_usize, 2, 4, 8, 12, 16, 20, 24] {
        results.push(averaged_benchmark(
            &format!("RwLock t={t}"), t, batch,
            &train, &test, &labels, epochs, attempts,
            || parallel_rwlock::train(&train, epochs, lr, t, batch, show_epochs),
            |w| sequential::infer(&test, w),
        ));
    }
    let rwlock_end = results.len();

    print_main_table(&mut results);

    let rayon_indices: Vec<usize> = (rayon_start..rayon_end).collect();
    print_batch_scalability_table(&results, &rayon_indices, "Rayon (batch 32 -> 128 -> 512)");

    let rwlock_indices: Vec<usize> = (rwlock_start..rwlock_end).collect();
    print_thread_scalability_table(&results, &rwlock_indices, "RwLock (1 -> 2 -> 4 -> 8 -> 12 -> 16 -> 20 -> 24 threads)");
}
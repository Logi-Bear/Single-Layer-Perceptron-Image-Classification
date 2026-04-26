# Benchmark Results

## Hardware

### Device 1 — Laptop (Razer Blade 15)
- CPU: Intel Core i7-8750H
- Cores: 6 cores / 12 threads
- RAM: 16 GB DDR4-2667 MHz

### Device 2 — Desktop
- CPU: Intel Core i9 12900k
- Cores: 16 cores (8 Performance + 8 Efficient) + 24 threads
- RAM: 32 GB (2× 16GB TEAMGROUP UD4-3200 DDR4)

## Configuration
- Dataset: 60,000 train / 10,000 test (MNIST)
- Epochs: 15
- Learning rate: 0.01
- Attempts averaged: 5

---

## Device 1 Results — Laptop (Razer Blade 15)

| Method      | Threads | Batch | Train |  Infer  | Accuracy | Speedup | Efficiency | Overhead |  Throughput  |
|-------------|---------|-------|-------|---------|----------|---------|------------|----------|--------------|
|Sequential:  |        1|    N/A|  7.34s|  79.57ms|    86.67%|    1.00x|      100.0%|    0.000s|  122,577img/s|
|Rayon b=32:  |       12|     32|  1.05s|   9.00ms|    76.91%|    7.01x|       58.4%|    0.436s|  859,188img/s|
|Rayon b=128: |       12|    128|  0.99s|   8.82ms|    83.32%|    7.40x|       61.7%|    0.380s|  907,659img/s|
|Rayon b=512: |       12|    512|  0.97s|   8.78ms|    79.28%|    7.58x|       63.2%|    0.356s|  929,491img/s|
|RwLock t=1:  |        1|    128| 11.02s|  80.51ms|    87.57%|    0.67x|       66.6%|    0.000s|   81,782img/s|
|RwLock t=2:  |        2|    128|  6.13s|  80.32ms|    89.59%|    1.20x|       59.9%|    2.454s|  146,929img/s|
|RwLock t=4:  |        4|    128|  3.83s|  84.73ms|    90.54%|    1.92x|       47.9%|    1.994s|  235,735img/s|
|RwLock t=8:  |        8|    128|  2.79s|  98.97ms|    91.22%|    2.63x|       32.9%|    1.876s|  322,111img/s|
|RwLock t=12: |       12|    128|  2.64s|  98.48ms|    90.83%|    2.78x|       23.2%|    2.030s|  340,703img/s|

---

## Device 2 Results — Desktop

| Method      | Threads | Batch | Train |  Infer  | Accuracy | Speedup | Efficiency | Overhead |  Throughput  |
|-------------|---------|-------|-------|---------|----------|---------|------------|----------|--------------|
|Sequential:  |        1|    N/A|  2.92s|  31.45ms|    87.04%|    1.00x|      100.0%|    0.000s|  313,998img/s|
|Rayon b=32:  |       24|     32|  0.90s|   9.26ms|    79.11%|    3.27x|       13.6%|    0.774s|1,111,514img/s|
|Rayon b=128: |       24|    128|  0.19s|   1.83ms|    79.12%|   15.30x|       63.7%|    0.069s|4,709,307img/s|
|Rayon b=512: |       24|    512|  0.19s|   1.82ms|    79.11%|   15.23x|       63.4%|    0.070s|4,689,522img/s|
|RwLock t=1:  |        1|    128|  3.97s|  30.26ms|    87.18%|    0.74x|       73.6%|    0.000s|  227,060img/s|
|RwLock t=2:  |        2|    128|  2.42s|  30.59ms|    89.45%|    1.21x|       60.3%|    0.962s|  372,461img/s|
|RwLock t=4:  |        4|    128|  1.68s|  30.87ms|    90.73%|    1.74x|       43.5%|    0.949s|  537,588img/s|
|RwLock t=8:  |        8|    128|  1.36s|  30.85ms|    91.20%|    2.16x|       27.0%|    0.990s|  664,578img/s|
|RwLock t=12: |       12|    128|  1.19s|  30.80ms|    89.34%|    2.46x|       20.5%|    0.944s|  757,652img/s|
|RwLock t=16: |       16|    128|  1.07s|  30.76ms|    90.36%|    2.73x|       17.1%|    0.887s|  841,405img/s|
|RwLock t=20: |       20|    128|  1.05s|  30.73ms|    88.97%|    2.80x|       14.0%|    0.899s|  861,362img/s|
|RwLock t=24: |       24|    128|  1.01s|  30.79ms|    87.85%|    2.90x|       12.1%|    0.888s|  891,629img/s|

---
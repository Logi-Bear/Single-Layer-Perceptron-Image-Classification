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

| Method      | Threads | Batch |  Train |    Infer | Accuracy | Speedup | Efficiency | Overhead |     Throughput |
|-------------|---------|-------|--------|----------|----------|---------|------------|----------|----------------|
| Sequential  |       1 |   N/A |  7.27s |  78.94ms |   86.96% |   1.00x |     100.0% |   0.000s |  123,772 img/s |
| Rayon b=32  |      12 |    32 |  1.40s |  12.11ms |   79.24% |   5.19x |      43.3% |   0.796s |  654,794 img/s |
| Rayon b=64  |      12 |    64 |  1.21s |  11.19ms |   79.09% |   6.01x |      50.1% |   0.604s |  751,381 img/s |
| Rayon b=96  |      12 |    96 |  1.13s |  10.70ms |   79.14% |   6.43x |      53.6% |   0.524s |  801,023 img/s |
| Rayon b=128 |      12 |   128 |  1.05s |  10.04ms |   79.14% |   6.92x |      57.7% |   0.444s |  853,376 img/s |
| Rayon b=512 |      12 |   512 |  1.05s |   9.95ms |   79.23% |   6.92x |      57.7% |   0.443s |  857,287 img/s |
| RwLock t=1  |       1 |   128 | 10.59s |  80.04ms |   87.16% |   0.69x |      68.7% |   0.000s |   84,997 img/s |
| RwLock t=2  |       2 |   128 |  6.23s |  80.96ms |   89.47% |   1.17x |      58.4% |   2.595s |  144,565 img/s |
| RwLock t=4  |       4 |   128 |  4.06s |  88.45ms |   90.73% |   1.79x |      44.7% |   2.243s |  221,852 img/s |
| RwLock t=8  |       8 |   128 |  2.80s |  97.54ms |   91.20% |   2.60x |      32.4% |   1.891s |  321,974 img/s |
| RwLock t=12 |      12 |   128 |  2.66s |  98.34ms |   89.30% |   2.73x |      22.8% |   2.054s |  338,551 img/s |

---

## Device 2 Results — Desktop

| Method      | Threads | Batch | Train |  Infer  | Accuracy | Speedup | Efficiency | Overhead |  Throughput  |
|-------------|---------|-------|-------|---------|----------|---------|------------|----------|--------------|
|Sequential:  |        1|    N/A|  2.92s|  31.45ms|    87.04%|    1.00x|      100.0%|    0.000s|  313,998img/s|
| Rayon b=32  |       24|     32|  0.21s|   1.84ms|    79.08%|   13.90x|       57.9%|    0.088s|4,302,599img/s|
| Rayon b=64  |       24|     64|  0.20s|   1.83ms|    79.07%|   14.60x|       60.8%|    0.078s|4,525,409img/s|
| Rayon b=96  |       24|     96|  0.20s|   1.83ms|    79.19%|   14.60x|       60.8%|    0.078s|4,600,017img/s|
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
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
|Sequential:  |        1|    N/A|  2.83s|  30.10ms|    86.90%|    1.00x|      100.0%|    0.000s|  317,981img/s|
|Rayon b=32:  |       24|     32|  0.21s|   1.83ms|    80.83%|   13.47x|       56.1%|    0.092s|4,287,662img/s|
|Rayon b=128: |       24|    128|  0.19s|   1.83ms|    80.14%|   14.84x|       61.8%|    0.073s|4,717,902img/s|
|Rayon b=512: |       24|    512|  0.19s|   1.82ms|    78.10%|   14.59x|       60.8%|    0.076s|4,643,296img/s|
|RwLock t=1:  |        1|    128|  4.49s|  30.26ms|    86.99%|    0.63x|       63.0%|    0.000s|  200,685img/s|
|RwLock t=2:  |        2|    128|  2.50s|  30.61ms|    89.28%|    1.13x|       56.5%|    1.090s|  359,772img/s|
|RwLock t=4:  |        4|    128|  1.70s|  30.90ms|    90.68%|    1.66x|       41.6%|    0.993s|  530,601img/s|
|RwLock t=8:  |        8|    128|  1.28s|  31.21ms|    91.27%|    2.22x|       27.7%|    0.924s|  704,767img/s|
|RwLock t=12: |       12|    128|  1.18s|  31.17ms|    89.76%|    2.41x|       20.1%|    0.940s|  766,056img/s|
|RwLock t=16: |       16|    128|  1.04s|  31.01ms|    91.16%|    2.72x|       17.0%|    0.863s|  865,528img/s|
|RwLock t=20: |       20|    128|  1.01s|  31.02ms|    89.53%|    2.81x|       14.0%|    0.867s|  892,466img/s|
|RwLock t=24: |       24|    128|  0.97s|  30.95ms|    86.08%|    2.92x|       12.2%|    0.850s|  929,641img/s|

---
# Benchmark Results

## Hardware

### Device 1 — Laptop (Razer Blade 15)
- CPU: Intel Core i7-8750H
- Cores: 6 physical / 12 logical
- RAM: 16 GB DDR4-2667 MHz
- OS: Windows 10 Home (64-bit)

### Device 2 — Desktop
- CPU: [model]
- Cores: [physical] / [logical]
- RAM: [amount]
- OS: [OS]

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
| Sequential  | 1       | N/A   |       |       |          |         |            |             |
| Rayon b=32  | [max]   | 32    |       |       |          |         |            |             |
| Rayon b=128 | [max]   | 128   |       |       |          |         |            |             |
| Rayon b=512 | [max]   | 512   |       |       |          |         |            |             |
| RwLock t=1  | 1       | 128   |       |       |          |         |            |             |
| RwLock t=2  | 2       | 128   |       |       |          |         |            |             |
| RwLock t=4  | 4       | 128   |       |       |          |         |            |             |
| RwLock t=8  | 8       | 128   |       |       |          |         |            |             |
| RwLock t=12 | 12      | 128   |       |       |          |         |            |             |

---
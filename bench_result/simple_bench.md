`DIMACS_brock200_2`に対しての各アルゴリズムの実行時間

## Bron-Kerbosch (without pivoting)

### 頂点集合をHashSetで管理

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run` | 59.300 ± 0.898 | 58.480 | 61.174 | 1.00 |

### 頂点集合をBitSetで管理

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run` | 25.587 ± 0.538 | 24.872 | 26.760 | 1.00 |

## Bron-Kerbosch (with pivoting)

### 頂点集合をHashSetで管理

ただし，ピボット選択はランダム

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run` | 24.492 ± 0.375 | 23.784 | 24.941 | 1.00 |

### 頂点集合をBitSetで管理

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo run` | 10.253 ± 0.091 | 10.107 | 10.357 | 1.00 |

import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path

DIR = Path(__file__).resolve().parent

# timestamp_with_pivot_bitset.txtを読み取り、グラフを作成する
# 作成したグラフはbench_result/simple_bench/plot_timestamp.pngに保存される

# ファイルを読み取り、データを取得する
with open(DIR / 'timestamp_with_pivot_bitset.txt', 'r') as f:
    lines = f.readlines()
    lines = [line.strip() for line in lines]
    lines = [line.split() for line in lines]
    lines = [[int(line[0]), int(line[1])] for line in lines]

# データをグラフにプロットする
x = [line[0] for line in lines]
y = [line[1] for line in lines]
plt.plot(x, y)

# グラフのタイトル、軸ラベルを設定する
plt.title('timestamp with pivot bitset')
plt.xlabel('pivot bitset size')
plt.ylabel('time [ms]')
plt.grid(True)

# グラフを表示する
plt.show()

# グラフを保存する
plt.savefig('plot_timestamp.png')

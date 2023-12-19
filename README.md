# maximal-clique-enumeration

極大クリークの列挙アルゴリズム

## ディレクトリ構成

```
maximal-clique-enumeration
  ├ bench_result
  │   └ simple_bench                   # brock200_2のみでのベンチマーク結果
  ├ (DIMACS_subset_ascii)              # テストケース
  │   ├ brock200_2.clq
  │   └ ...
  ├ src
  │   ├ bin
  │   │   ├ simple_bench.rs            # brock200_2のみでのベンチマーク
  │   │   └ simple_bench_timestamp.rs  # タイムスタンプを記録
  │   ├ algorithms.rs                  # 列挙アルゴリズムの実装
  │   ├ algorithms.rs                  # 列挙アルゴリズムの実装（タイムスタンプを記録）
  │   ├ input.rs                       # 入力の受け取り
  │   └ main.rs                        # ベンチマークの実行
  └ util
      └ setup.sh                       # テストケースのダウンロード
```

# maximal-clique-enumeration

極大クリークの列挙アルゴリズム

## ディレクトリ構成

```
maximal-clique-enumeration
  ├ (DIMACS_subset_ascii) # テストケース
  │   ├ brock200_2.clq
  │   └ ...
  ├ src
  │   ├ algorithms.rs # 列挙アルゴリズムの実装
  │   ├ bench.rs      # 時間の計測
  │   ├ input.rs      # 入力の受け取り
  │   └ main.rs       # ベンチマークの実行
  └ util
      └ setup.sh # テストケースのダウンロード
```

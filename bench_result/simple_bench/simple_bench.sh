algos=(
    "without_pivot_hashset"
    "without_pivot_bitset"
    "with_pivot_hashset"
    "with_pivot_bitset"
)

for algo in ${algos[@]}; do
    # ベンチマークを実行
    hyperfine "cargo run --bin simple_bench $algo" --export-markdown "$algo.md"
done

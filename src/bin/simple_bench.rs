use maximal_clique_enumeration::{
    algorithms::{
        bron_kerbosch_with_pivoting, bron_kerbosch_with_pivoting_bitset,
        bron_kerbosch_without_pivoting, bron_kerbosch_without_pivoting_bitset,
    },
    input::{GraphInput, VertexBitSet, VertexSet},
};

const DIMACS_TESTCASE: &str =
    "/Users/komotokenta/Docker/maximal-clique-enumeration/DIMACS_subset_ascii/brock200_2.clq";

fn main() {
    let algo_type = std::env::args().nth(1).unwrap();

    let size = match &algo_type[..] {
        "without_pivot_hashset" => without_pivot_hashset(),
        "without_pivot_bitset" => without_pivot_bitset(),
        "with_pivot_hashset" => with_pivot_hashset(),
        "with_pivot_bitset" => with_pivot_bitset(),
        _ => unreachable!(),
    };

    println!("maximal clique count: {size}");
}

fn without_pivot_hashset() -> usize {
    let input_1 = GraphInput::input_from_ascii_file(DIMACS_TESTCASE);
    let graph = input_1.into_adjacent_set();

    let mut res = vec![];
    bron_kerbosch_without_pivoting(
        &mut VertexSet::default(),
        &mut (0..input_1.v_size).collect(),
        &mut VertexSet::default(),
        &graph,
        &mut res,
    );

    res.len()
}

fn without_pivot_bitset() -> usize {
    let input_1 = GraphInput::input_from_ascii_file(DIMACS_TESTCASE);
    let graph = input_1.into_adjacent_bitset();

    let mut res = vec![];
    bron_kerbosch_without_pivoting_bitset(
        &mut VertexBitSet::default(),
        &mut (0..input_1.v_size).collect(),
        &mut VertexBitSet::default(),
        &graph,
        &mut res,
    );

    res.len()
}

fn with_pivot_hashset() -> usize {
    let input_1 = GraphInput::input_from_ascii_file(DIMACS_TESTCASE);
    let graph = input_1.into_adjacent_set();

    let mut res = vec![];
    bron_kerbosch_with_pivoting(
        &mut VertexSet::default(),
        &mut (0..input_1.v_size).collect(),
        &mut VertexSet::default(),
        &graph,
        &mut res,
    );

    res.len()
}

fn with_pivot_bitset() -> usize {
    let input_1 = GraphInput::input_from_ascii_file(DIMACS_TESTCASE);
    let graph = input_1.into_adjacent_bitset();

    let mut res = vec![];
    bron_kerbosch_with_pivoting_bitset(
        &mut VertexBitSet::default(),
        &mut (0..input_1.v_size).collect(),
        &mut VertexBitSet::default(),
        &graph,
        &mut res,
    );

    res.len()
}

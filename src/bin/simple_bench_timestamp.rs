use std::time;

use maximal_clique_enumeration::{
    input::{GraphInput, VertexBitSet, VertexSet},
    timestamp::bron_kerbosch_with_pivoting_bitset,
};

const DIMACS_TESTCASE: &str =
    "/Users/komotokenta/Docker/maximal-clique-enumeration/DIMACS_subset_ascii/brock200_2.clq";

fn main() {
    let algo_type = std::env::args().nth(1).unwrap();

    match &algo_type[..] {
        // "without_pivot_hashset" => without_pivot_hashset(),
        // "without_pivot_bitset" => without_pivot_bitset(),
        // "with_pivot_hashset" => with_pivot_hashset(),
        "with_pivot_bitset" => with_pivot_bitset(),
        _ => unreachable!(),
    }
}

// fn without_pivot_hashset() {
//     let input_1 = GraphInput::input_from_ascii_file(DIMACS_TESTCASE);
//     let graph = input_1.into_adjacent_set();
//     let start = time::Instant::now();

//     let mut res = vec![];
//     bron_kerbosch_without_pivoting(
//         &mut VertexSet::default(),
//         &mut (0..input_1.v_size).collect(),
//         &mut VertexSet::default(),
//         &graph,
//         &start,
//         &mut res,
//     );

//     for stamp in res {
//         println!("{:?}", stamp);
//     }
// }

// fn without_pivot_bitset() {
//     let input_1 = GraphInput::input_from_ascii_file(DIMACS_TESTCASE);
//     let graph = input_1.into_adjacent_bitset();

//     let mut res = vec![];
//     bron_kerbosch_without_pivoting_bitset(
//         &mut VertexBitSet::default(),
//         &mut (0..input_1.v_size).collect(),
//         &mut VertexBitSet::default(),
//         &graph,
//         &mut res,
//     );

//     for timestamp in res {}
// }

// fn with_pivot_hashset() {
//     let input_1 = GraphInput::input_from_ascii_file(DIMACS_TESTCASE);
//     let graph = input_1.into_adjacent_set();

//     let mut res = vec![];
//     bron_kerbosch_with_pivoting(
//         &mut VertexSet::default(),
//         &mut (0..input_1.v_size).collect(),
//         &mut VertexSet::default(),
//         &graph,
//         &mut res,
//     );
// }

fn with_pivot_bitset() {
    let input_1 = GraphInput::input_from_ascii_file(DIMACS_TESTCASE);
    let graph = input_1.into_adjacent_bitset();
    let start = time::Instant::now();

    let mut res = vec![];
    bron_kerbosch_with_pivoting_bitset(
        &mut VertexBitSet::default(),
        &mut (0..input_1.v_size).collect(),
        &mut VertexBitSet::default(),
        &graph,
        &start,
        &mut res,
    );

    for (i, &stamp) in res.iter().enumerate() {
        println!("{} {}", i, stamp.as_nanos());
    }
}

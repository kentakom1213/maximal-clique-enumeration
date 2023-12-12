use maximal_clique_enumeration::input::{AdjList, AdjMatrix, GraphInput};

fn main() {
    let G = GraphInput::input_from_ascii_file("DIMACS_subset_ascii/brock200_2.clq");

    println!("{:?}", G.into_adjacent_list());
}

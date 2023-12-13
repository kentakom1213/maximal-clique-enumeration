use maximal_clique_enumeration::input::{AdjList, AdjMatrix, GraphInput};

const DIMACS_TESTCASES: [&str; 37] = [
    "DIMACS_subset_ascii/brock200_2.clq",
    "DIMACS_subset_ascii/brock200_4.clq",
    "DIMACS_subset_ascii/brock400_2.clq",
    "DIMACS_subset_ascii/brock400_4.clq",
    "DIMACS_subset_ascii/brock800_2.clq",
    "DIMACS_subset_ascii/brock800_4.clq",
    "DIMACS_subset_ascii/C125.9.clq",
    "DIMACS_subset_ascii/C250.9.clq",
    "DIMACS_subset_ascii/C500.9.clq",
    "DIMACS_subset_ascii/C1000.9.clq",
    "DIMACS_subset_ascii/C2000.5.clq",
    "DIMACS_subset_ascii/C2000.9.clq",
    "DIMACS_subset_ascii/C4000.5.clq",
    "DIMACS_subset_ascii/DSJC500_5.clq",
    "DIMACS_subset_ascii/DSJC1000_5.clq",
    "DIMACS_subset_ascii/gen200_p0.9_44.clq",
    "DIMACS_subset_ascii/gen200_p0.9_55.clq",
    "DIMACS_subset_ascii/gen400_p0.9_55.clq",
    "DIMACS_subset_ascii/gen400_p0.9_65.clq",
    "DIMACS_subset_ascii/gen400_p0.9_75.clq",
    "DIMACS_subset_ascii/hamming8-4.clq",
    "DIMACS_subset_ascii/hamming10-4.clq",
    "DIMACS_subset_ascii/keller4.clq",
    "DIMACS_subset_ascii/keller5.clq",
    "DIMACS_subset_ascii/keller6.clq",
    "DIMACS_subset_ascii/MANN_a27.clq",
    "DIMACS_subset_ascii/MANN_a45.clq",
    "DIMACS_subset_ascii/MANN_a81.clq",
    "DIMACS_subset_ascii/p_hat300-1.clq",
    "DIMACS_subset_ascii/p_hat300-2.clq",
    "DIMACS_subset_ascii/p_hat300-3.clq",
    "DIMACS_subset_ascii/p_hat700-1.clq",
    "DIMACS_subset_ascii/p_hat700-2.clq",
    "DIMACS_subset_ascii/p_hat700-3.clq",
    "DIMACS_subset_ascii/p_hat1500-1.clq",
    "DIMACS_subset_ascii/p_hat1500-2.clq",
    "DIMACS_subset_ascii/p_hat1500-3.clq",
];

fn main() {
    for &case in &DIMACS_TESTCASES {
        let graph = GraphInput::input_from_ascii_file(case);

        println!("{}: v:{}, e:{}", case, graph.v_size, graph.e_size);
    }
}

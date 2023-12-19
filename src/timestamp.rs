#![allow(non_snake_case)]

use crate::input::{AdjBitSet, AdjSet, VertexBitSet, VertexSet};
use bit_set::BitSet;
use itertools::Itertools;
use std::time;

// /// ピボットなしのBron-Kerbosch法
// /// - R: 極大クリークの部分グラフとしてすでに選んでいる頂点集合
// /// - P,X: R中の頂点全てに隣接している頂点集合の候補
// pub fn bron_kerbosch_without_pivoting(
//     R: &mut VertexSet,
//     P: &mut VertexSet,
//     X: &mut VertexSet,
//     G: &AdjSet,
//     start: &time::Instant,
//     res: &mut Vec<time::Duration>,
// ) {
//     if P.is_empty() && X.is_empty() {
//         res.push(start.elapsed());
//         return;
//     }
//     for &v in &P.clone() {
//         bron_kerbosch_without_pivoting(
//             &mut R.union(&VertexSet::from([v])).cloned().collect(),
//             &mut P.intersection(&G[v]).cloned().collect(),
//             &mut X.intersection(&G[v]).cloned().collect(),
//             G,
//             start,
//             res,
//         );
//         P.remove(&v);
//         X.insert(v);
//     }
// }

// /// ピボットなしのBron-Kerbosch法
// /// - R: 極大クリークの部分グラフとしてすでに選んでいる頂点集合
// /// - P,X: R中の頂点全てに隣接している頂点集合の候補
// pub fn bron_kerbosch_without_pivoting_bitset(
//     R: &mut VertexBitSet,
//     P: &mut VertexBitSet,
//     X: &mut VertexBitSet,
//     G: &AdjBitSet,
//     res: &mut Vec<time::Instant>,
// ) {
//     if P.is_empty() && X.is_empty() {
//         res.push(time::Instant::now());
//         return;
//     }
//     for v in &P.clone() {
//         bron_kerbosch_without_pivoting_bitset(
//             &mut R.union(&VertexBitSet::from_iter([v])).collect(),
//             &mut P.intersection(&G[v]).collect(),
//             &mut X.intersection(&G[v]).collect(),
//             G,
//             res,
//         );
//         P.remove(v);
//         X.insert(v);
//     }
// }

// /// ピボットありのBron-Kerbosch法
// /// - R: 極大クリークの部分グラフとしてすでに選んでいる頂点集合
// /// - P,X: R中の頂点全てに隣接している頂点集合の候補
// pub fn bron_kerbosch_with_pivoting(
//     R: &mut VertexSet,
//     P: &mut VertexSet,
//     X: &mut VertexSet,
//     G: &AdjSet,
//     res: &mut Vec<time::Instant>,
// ) {
//     if P.is_empty() && X.is_empty() {
//         res.push(time::Instant::now());
//         return;
//     }
//     let pivot_conflict = choose_pivot(&P, &X, &G);
//     for &v in &pivot_conflict {
//         bron_kerbosch_with_pivoting(
//             &mut R.union(&VertexSet::from([v])).cloned().collect(),
//             &mut P.intersection(&G[v]).cloned().collect(),
//             &mut X.intersection(&G[v]).cloned().collect(),
//             G,
//             res,
//         );
//         P.remove(&v);
//         X.insert(v);
//     }
// }

/// ピボットありのBron-Kerbosch法（BitSet版）
/// - R: 極大クリークの部分グラフとしてすでに選んでいる頂点集合
/// - P,X: R中の頂点全てに隣接している頂点集合の候補
pub fn bron_kerbosch_with_pivoting_bitset(
    R: &mut VertexBitSet,
    P: &mut VertexBitSet,
    X: &mut VertexBitSet,
    G: &AdjBitSet,
    start: &time::Instant,
    res: &mut Vec<time::Duration>,
) {
    if P.is_empty() && X.is_empty() {
        res.push(start.elapsed());
        return;
    }
    let pivot_conflict = choose_pivot_bitset(&P, &X, &G);
    for v in &pivot_conflict {
        bron_kerbosch_with_pivoting_bitset(
            &mut R.union(&VertexBitSet::from_iter([v])).collect(),
            &mut P.intersection(&G[v]).collect(),
            &mut X.intersection(&G[v]).collect(),
            G,
            start,
            res,
        );
        P.remove(v);
        X.insert(v);
    }
}

// /// ピボットとなる頂点を選択する
// /// - |P ∩ N(q)| が最大になるような q ∈ P ∪ X を探索し P \ N(q) を返す
// fn choose_pivot(P: &VertexSet, X: &VertexSet, G: &AdjSet) -> VertexSet {
//     // let mut max_intersection_size = 0;
//     // let mut res = VertexSet::default();
//     // let mut pivot = usize::MAX;
//     // for &q in P.union(&X) {
//     //     let tmp: VertexSet = P.intersection(&G[q]).cloned().collect();
//     //     if max_intersection_size < tmp.len() {
//     //         max_intersection_size = tmp.len();
//     //         res = P.difference(&G[q]).cloned().collect();
//     //         pivot = q;
//     //     }
//     // }
//     // res
//     let pivot = *P.union(&X).next().unwrap();
//     P.difference(&G[pivot]).cloned().collect()
// }

/// ピボットとなる頂点を選択する
/// - |P ∩ N(q)| が最大になるような q ∈ P ∪ X を探索し P \ N(q) を返す
fn choose_pivot_bitset(P: &VertexBitSet, X: &VertexBitSet, G: &AdjBitSet) -> VertexBitSet {
    // let mut max_intersection_size = 0;
    // let mut res = VertexSet::default();
    // let mut pivot = usize::MAX;
    // for &q in P.union(&X) {
    //     let tmp: VertexSet = P.intersection(&G[q]).cloned().collect();
    //     if max_intersection_size < tmp.len() {
    //         max_intersection_size = tmp.len();
    //         res = P.difference(&G[q]).cloned().collect();
    //         pivot = q;
    //     }
    // }
    // res
    let pivot = P.union(&X).next().unwrap();
    P.difference(&G[pivot]).collect()
}

// #[cfg(test)]
// mod test_algorithm {
//     use super::*;

//     #[test]
//     fn test_bron_kerbosch_without_pivoting() {
//         // グラフ
//         let G = vec![
//             VertexSet::from([1, 3, 6, 7]),
//             VertexSet::from([0, 2, 3, 6, 7]),
//             VertexSet::from([1, 3, 4, 5]),
//             VertexSet::from([0, 1, 2, 4, 6, 7]),
//             VertexSet::from([2, 3, 5, 6, 7]),
//             VertexSet::from([2, 4, 6]),
//             VertexSet::from([0, 1, 3, 4, 5, 7]),
//             VertexSet::from([0, 1, 3, 4, 6]),
//         ];

//         let mut res = vec![];
//         bron_kerbosch_without_pivoting(
//             &mut VertexSet::default(),
//             &mut (0..8).collect(),
//             &mut VertexSet::default(),
//             &G,
//             &mut res,
//         );

//         eprintln!("{:#?}", res);
//     }

//     #[test]
//     fn test_bron_kerbosch_without_pivoting_bitset() {
//         // グラフ
//         let G = vec![
//             VertexBitSet::from_iter([1, 3, 6, 7]),
//             VertexBitSet::from_iter([0, 2, 3, 6, 7]),
//             VertexBitSet::from_iter([1, 3, 4, 5]),
//             VertexBitSet::from_iter([0, 1, 2, 4, 6, 7]),
//             VertexBitSet::from_iter([2, 3, 5, 6, 7]),
//             VertexBitSet::from_iter([2, 4, 6]),
//             VertexBitSet::from_iter([0, 1, 3, 4, 5, 7]),
//             VertexBitSet::from_iter([0, 1, 3, 4, 6]),
//         ];

//         let mut res = vec![];
//         bron_kerbosch_without_pivoting_bitset(
//             &mut VertexBitSet::default(),
//             &mut (0..8).collect(),
//             &mut VertexBitSet::default(),
//             &G,
//             &mut res,
//         );

//         eprintln!("{:#?}", res);
//     }

//     #[test]
//     fn test_bron_kerbosch_with_pivoting() {
//         // グラフ
//         let G = vec![
//             VertexSet::from([1, 3, 6, 7]),
//             VertexSet::from([0, 2, 3, 6, 7]),
//             VertexSet::from([1, 3, 4, 5]),
//             VertexSet::from([0, 1, 2, 4, 6, 7]),
//             VertexSet::from([2, 3, 5, 6, 7]),
//             VertexSet::from([2, 4, 6]),
//             VertexSet::from([0, 1, 3, 4, 5, 7]),
//             VertexSet::from([0, 1, 3, 4, 6]),
//         ];

//         let mut res = vec![];
//         bron_kerbosch_with_pivoting(
//             &mut VertexSet::default(),
//             &mut (0..8).collect(),
//             &mut VertexSet::default(),
//             &G,
//             &mut res,
//         );

//         eprintln!("{:#?}", res);
//     }
// }

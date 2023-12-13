#![allow(non_snake_case)]

use crate::input::{AdjSet, VertexSet};

/// ピボットなしのBron-Kerbosch法
/// - R: 極大クリークの部分グラフとしてすでに選んでいる頂点集合
/// - P,X: R中の頂点全てに隣接している頂点集合の候補
pub fn bron_kerbosch_without_pivoting(
    R: &mut VertexSet,
    P: &mut VertexSet,
    X: &mut VertexSet,
    G: &AdjSet,
    res: &mut Vec<VertexSet>,
) {
    if P.is_empty() && X.is_empty() {
        res.push(R.clone());
        return;
    }
    for &v in &P.clone() {
        bron_kerbosch_without_pivoting(
            &mut R.union(&VertexSet::from([v])).cloned().collect(),
            &mut P.intersection(&G[v]).cloned().collect(),
            &mut X.intersection(&G[v]).cloned().collect(),
            G,
            res,
        );
        P.remove(&v);
        X.insert(v);
    }
}

#[cfg(test)]
mod test_algorithm {
    use super::*;

    #[test]
    fn test_bron_kerbosch_without_pivoting() {
        // グラフ
        let G = vec![
            VertexSet::from([1, 3, 6, 7]),
            VertexSet::from([0, 2, 3, 6, 7]),
            VertexSet::from([1, 3, 4, 5]),
            VertexSet::from([0, 1, 2, 4, 6, 7]),
            VertexSet::from([2, 3, 5, 6, 7]),
            VertexSet::from([2, 4, 6]),
            VertexSet::from([0, 1, 3, 4, 5, 7]),
            VertexSet::from([0, 1, 3, 4, 6]),
        ];

        let mut res = vec![];
        bron_kerbosch_without_pivoting(
            &mut VertexSet::default(),
            &mut (0..8).collect(),
            &mut VertexSet::default(),
            &G,
            &mut res,
        );

        eprintln!("{:#?}", res);
    }
}

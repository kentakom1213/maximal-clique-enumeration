use std::{collections::HashSet, fs::read_to_string};

/// 頂点集合
pub type VertexSet = HashSet<usize>;

/// 隣接行列
pub type AdjMatrix = Vec<Vec<bool>>;

/// 隣接リスト（ベクタ）
pub type AdjList = Vec<Vec<usize>>;

/// 隣接リスト（集合型）
pub type AdjSet = Vec<VertexSet>;

/// 入力受け取り用構造体
#[derive(Debug)]
pub struct GraphInput {
    pub v_size: usize,
    pub e_size: usize,
    pub edges: Vec<(usize, usize)>,
}

impl GraphInput {
    /// ファイル名を指定してグラフを読み込む
    pub fn input_from_ascii_file(filename: &str) -> Self {
        // ファイルの読み込み
        let Ok(file) = read_to_string(filename) else {
            panic!("No such file: {}", filename);
        };

        let (mut v_size, mut e_size) = (0, 0);
        let mut edges = vec![];

        // 辺の追加
        for line in file.lines() {
            if line.starts_with('p') {
                if let [_, _, v_str, e_str] = line.split_whitespace().collect::<Vec<&str>>()[..] {
                    v_size = v_str.parse::<usize>().unwrap();
                    e_size = e_str.parse::<usize>().unwrap();
                }
            }
            if line.starts_with('e') {
                if let [_, u_str, v_str] = line.split_ascii_whitespace().collect::<Vec<&str>>()[..]
                {
                    // 0-indexedに修正
                    let u = u_str.parse::<usize>().unwrap() - 1;
                    let v = v_str.parse::<usize>().unwrap() - 1;
                    edges.push((u, v));
                }
            }
        }
        Self {
            v_size,
            e_size,
            edges,
        }
    }

    /// 隣接行列形式に変換
    pub fn into_adjacent_matrix(&self) -> AdjMatrix {
        self.edges.iter().fold(
            vec![vec![false; self.v_size]; self.v_size],
            |mut m, &(u, v)| {
                m[u][v] = true;
                m[v][u] = true;
                m
            },
        )
    }

    /// 隣接リスト形式に変換
    pub fn into_adjacent_list(&self) -> AdjList {
        self.edges
            .iter()
            .fold(vec![vec![]; self.v_size], |mut g, &(u, v)| {
                g[u].push(v);
                g[v].push(u);
                g
            })
    }

    /// 隣接リスト（セット）形式に変換
    pub fn into_adjacent_set(&self) -> AdjSet {
        self.edges
            .iter()
            .fold(vec![VertexSet::default(); self.v_size], |mut g, &(u, v)| {
                g[u].insert(v);
                g[v].insert(u);
                g
            })
    }
}

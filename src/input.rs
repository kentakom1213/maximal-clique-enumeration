use std::fs::read_to_string;

#[derive(Debug)]
pub struct GraphInput {
    pub v: usize,
    pub e: usize,
    pub edges: Vec<(usize, usize)>,
}

impl GraphInput {
    pub fn input_from_ascii_file(filename: &str) -> Self {
        let file = read_to_string(filename).ok().unwrap();

        let (mut v, mut e) = (0, 0);
        let mut edges = vec![];

        // 読み込み
        for line in file.lines() {
            if line.starts_with('p') {
                if let [_, _, v_str, e_str] = line.split_whitespace().collect::<Vec<&str>>()[..] {
                    v = v_str.parse::<usize>().unwrap();
                    e = e_str.parse::<usize>().unwrap();
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

        Self { v, e, edges }
    }
}

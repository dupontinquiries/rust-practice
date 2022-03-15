
use std::{collections::HashSet, hash::Hash, collections::HashMap};//, collections::OrderedMap};

struct G {
    n: HashSet<String>,
    m: HashSet<(u32, String, String)>,
}

type Elst = HashMap<String, bool>;
type Clk = HashMap<String, u32>;

fn explore(r: String, g: G, visited: &Elst, pre: &Clk, post: &Clk, &clock: u32) {
    visited.insert(r, true);
    pre.insert(r, clock);
    // pre[r] = clock
    // ++clock
    // for each edge (a, b) in E
    // 	if not visited[b]
    // 		explore(G, b)
    post.insert(r, clock);
    clock += 1;
    // .
}

fn dfs(r: String, g: G) {
    let mut visited = Elst::with_capacity( (g.m.len() - 1) as usize );
    visited.insert(r, true);
}

fn main() {
    println!("Hello, world!");
}

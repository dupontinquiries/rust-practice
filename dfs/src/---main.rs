use std::{collections::HashMap, collections::HashSet, hash::Hash};

struct G {
    n: HashSet<String>,
    m: HashSet<(u32, String, String)>,
}

type Vmp = HashMap<String, bool>;
type Clk = HashMap<String, u32>;

fn explore(r: String, g: &G, visited: &Vmp, pre: &Clk, post: &Clk, clock: &u32) {
    visited.insert(r, true);
    pre.insert(r, *clock);
    // pre[r] = clock
    // ++clock
    // for each edge (a, b) in E
    // 	if not visited[b]
    // 		explore(G, b)
    post.insert(r, *clock);
    *clock += 1;
}

fn dfs(r: String, g: G) {
    let mut visited = Vmp::with_capacity(g.n.len() as usize);
    visited.insert(r, true);
    let mut pre = Clk::with_capacity(g.n.len() as usize);
    let mut post = Clk::with_capacity(g.n.len() as usize);
    let mut clock = 0;
    for key in g.n.into_iter() {
        if !visited.contains_key(&key) {
            explore(key, &g, &visited, &pre, &post, &clock);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

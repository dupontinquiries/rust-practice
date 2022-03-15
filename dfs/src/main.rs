use std::{collections::HashMap, collections::HashSet, hash::Hash};

pub struct Solution<T> {
    // visited: HashSet<T>,
    // pre: HashMap<T, f64>,
    // post: HashMap<T, f64>,
    // clock: u64,
}

impl Solution<T> {

    pub fn dfs_explore<T>(n: T, g: &HashMap<T, HashSet<T>>) {
        visited.insert(n);
        pre.insert(n, clock);
        clock += 1;
        // match g.get(n) {
        //     Some(hs) => dfs_explore(v, &g),
        //     None => println!("{} is unreviewed.", book)
        // }
        for v in g.find(n).into_iter() {
            if !visited.contains_key(&v) {
                dfs_explore(v, &g)
            }
        }
        post.insert(n, clock);
        clock += 1;
    }

    pub fn dfs<T>(r: T, g: &HashMap<T, HashSet<T>>) {
        // let mut visited = HashSet<T>::with_capacity(g.n.len() as usize);
        // let mut pre = HashMap<T, u32>::with_capacity(g.n.len() as usize);
        // let mut post = HashMap<T, u32>::with_capacity(g.n.len() as usize);
        // let mut clock = 0 as u32;

        let mut visited: HashSet<T> = HashSet<T>::new();
        let mut pre: HashMap<T, f64> = HashMap<T, f64>::new();
        let mut post: HashMap<T, f64> = HashMap<T, f64>::new();
        let mut clock: u64 = 0;

        visited.insert(r);
        
        for (u, neighbors) in g.into_iter() {
            if !visited.contains_key(&u) {
                dfs_explore(u, g);
            }
        }
    }

}


fn main() {
    println!("Hello, world!");
}

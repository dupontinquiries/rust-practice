use std::{collections::HashMap, collections::HashSet, hash::Hash};

pub struct Solution<T> where T: Hash + Eq {}

impl Solution<T> {

    type Clk = HashMap<T, f64>;
    type Adl = HashMap<T, HashSet<T>>;

    pub fn dfs_explore<T>(n: T, g: Adl, pre: Clk, post: Clk) {

        visited.insert(n);
        pre.insert(n, clock);
        clock += 1;
        // match g.get(n) {
        //     Some(hs) => dfs_explore(v, &g),
        //     None => println!("{} is unreviewed.", book)
        // }

        // map.iter().find_map(|(key, &val)| if val == value { Some(key) } else { None });

        for v in g.find(n).into_iter() {
            if !visited.contains_key(&v) {
                dfs_explore(v, &g);
            }
        }
        post.insert(n, clock);
        clock += 1;

    }

    pub fn dfs<T>(r: T, g: &HashMap<T, HashSet<T>>) {

        let mut visited: HashSet<T> = HashSet<T>::new();
        let mut pre: Clk::new();
        let mut post: Clk::new();
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

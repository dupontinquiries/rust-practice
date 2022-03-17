//
// use crate std::collections::HashSet;
// use crate std::collections::HashMap;

// type Adl = HashMap<String, HashSet<String>>;
//
// struct Adl<N> {
//     l: HashMap<N, HashSet<N>>,
// }

use crate std::collections::HashSet;
use crate std::collections::HashMap;
// use std::collections::hash_map::Entry;

struct Adl<N> {
    adl: HashMap<N, HashSet<N>>,
}

impl Adl<N> {
    pub fn add_directed_edge(a: N, b: N) {
        let neighbors = adl.entry(a).or_insert(HashSet<N>::New());
        if !neighbors.contains(b) {
            *neighbors.insert(b);
        }
        if !adl.contains_key(b) {
            adl.insert((b, HashSet<N>::New()));
        }
    }
}

pub fn create_graph() -> Adl<String> {
    Adl<String> {
        adl: HashMap<String, HashSet<String>>::New(),
    };
}

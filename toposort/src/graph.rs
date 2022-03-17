


pub mod graph {
    pub use crate::std::collections::HashMap;
    pub use crate::std::collections::HashSet;

    struct Adl {
        adl: HashMap<String, HashSet<String>>,
    }

    impl Adl {
        pub fn add_directed_edge(a: N, b: N) {
            let neighbors = adl.entry(a).or_insert(HashSet<String>::New());
            if !neighbors.contains(b) {
                *neighbors.insert(b);
            }
            if !adl.contains_key(b) {
                adl.insert((b, HashSet<String>::New()));
            }
        }
    }

    pub fn create_graph() -> Adl {
        Adl {
            adl: HashMap<String, HashSet<String>>::New(),
        };
    }
}

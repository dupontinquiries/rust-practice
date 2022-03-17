
// use crate::graph::Adl;
use crate::graph::Graph;

mod graph;

// use crate std::collections::HashSet;
//
// type Adl<N> = HashMap<N, HashSet<N>>;
//
// fn toposort<N>(g: Adl<N>) -> Adl<N> {
//     r = reverse_graph(g);
//     let dfs_res = HashMap<N, [u64; 2]>::with_capacity(g.len() as usize);
//     while !r.empty() {
//         g //return g for now
//         // remove highest post num
//     }
// }

fn main() {
    println!("running tests...");
    let mut g1 = graph::create_graph();
    g1.add_directed_edge("a","b");
    g1.add_directed_edge("a","c");
    g1.add_directed_edge("b","d");
    // let g1 = create_adl(vec["a","b","c","d","e"], vec[""] true)
    println!("finished tests");
}

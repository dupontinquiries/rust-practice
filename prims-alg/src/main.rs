
// use std::collections::HashMap;
// use multimap::MultiMap;
// minqueue<(cost: i32, u: u32, v: u32)>


// fn prim(root: u32, n: Vec<u32>, m: Vec<[u32; 2]>) -> (Vec<u32>, Vec<[u32; 2]>) {
// root: u32
// n: number of nodes in graph
// m: vec of (cost, u, v)
// return: vec of (cost, u, v)
fn prim_mst(root: u32, n: u32, m: Vec<(i32, u32, u32)>) -> Vec<(i32, u32, u32)> {
    // create an empty vec to store the edges in as the mst expands
    let mut mst_edges = Vec::with_capacity( (n - 1) as usize );
    // create a vec to store the nodes in as the mst expands
    let mut visited = vec![false; n as usize];
    // add root node to mst
    visited[root as usize] = true;
    // create a sorted (cost, ascending) queue of edges
    let mut m_sorted = m.clone(); // https://stackoverflow.com/questions/21369876/what-is-the-idiomatic-rust-way-to-copy-clone-a-vector-in-a-parameterized-functio
    m_sorted.sort_by(|a, b| a.0.cmp(&b.0)); // https://stackoverflow.com/questions/40091161/sorting-a-vector-of-tuples-needs-a-reference-for-the-second-value
    // iterate through map until removed n-1 edges
    // let mut count = 0 as u32;
    while mst_edges.len() < n as usize {
    	// find lowest edge that has only one endpoint in visited
        let mut found_option = false;
    	for i in 0..m_sorted.len() {
            // println!("m_sorted: {:?}", m_sorted);
            let u = m_sorted[i].1;
            let v = m_sorted[i].2;
            let bool1 = visited[u as usize];
            let bool2 = visited[v as usize];
            // if only one node of edge in mst, add edge to mst_edges & remove from queue
            if (bool1 && !bool2) || (!bool1 && bool2) {
                // mark other node as visited
                if bool1 {
                    visited[v as usize] = true;
                } else {
                    visited[u as usize] = true;
                }
                // add edge to mst_edges
                mst_edges.push( m_sorted[i as usize] );
                // remove edge from queue
                m_sorted.remove(i);
                // declare a continue
                found_option = true;
                break;
            }
        }
        if !found_option {
            break;
        }
    }
    mst_edges
}

fn main() {
    // (weight, u, v)
    println!("running tests...");
    // 0
    assert_eq!(
        prim_mst(0, 1, vec![]),
        vec![]
    );
    // 1
    assert_eq!(
        prim_mst(0, 4, vec![(1, 0, 1), (1, 1, 2), (1, 2, 3)]),
        vec![(1, 0, 1), (1, 1, 2), (1, 2, 3)]
    );
    // 2
    assert_eq!(
        prim_mst(0, 4, vec![(2, 0, 1), (2, 1, 2), (2, 2, 3), (1, 0, 1), (1, 1, 2), (1, 2, 3)]),
        vec![(1, 0, 1), (1, 1, 2), (1, 2, 3)]
    );
    // 3
    assert_eq!(
        prim_mst(0, 4, vec![(1, 0, 1), (2, 1, 2), (1, 2, 3), (1, 0, 3)]),
        vec![(1, 0, 1), (1, 0, 3), (1, 2, 3)]
    );
    // 4
    assert_eq!(
        prim_mst(0, 6, vec![(4, 3, 5), (1, 0, 1), (1, 1, 2), (1, 2, 3), (1, 0, 5), (1, 4, 5)]),
        vec![(1, 0, 1), (1, 1, 2), (1, 2, 3), (1, 0, 5), (1, 4, 5)]
    );
    // 5
    assert_eq!(
        prim_mst(0, 9, vec![(1, 0, 2), (7, 2, 3), (11, 1, 3), (3, 3, 4), (6, 1, 4), (5, 0, 1), (10, 0, 5), (4, 5, 7), (9, 7, 8), (8, 6, 8), (12, 5, 6), (2, 0, 6)]),
        vec![(1, 0, 2), (2, 0, 6), (5, 0, 1), (6, 1, 4), (3, 3, 4), (8, 6, 8), (9, 7, 8), (4, 5, 7)]
    );
    // note, indices 1 and 2 are interchangeable for any tuple (weight, u, v) since we assume an undirected graph
    // also, we can deduce whether or not the algorithm was able to build a tree by checking to see if the return vec's len = n - 1
    println!("finished tests");
}

fn debug_print() {
    // (weight, u, v)
    println!("running tests...");
    println!("{:?}", 0);
    println!("{:?}", prim_mst(0, 1, vec![]));
    // expected: []
    println!("{:?}", 1);
    println!("{:?}", prim_mst(0, 4, vec![(1, 0, 1), (1, 1, 2), (1, 2, 3)]));
    // expected: [(1, 0, 1), (1, 1, 2), (1, 2, 3)]
    println!("{:?}", 2);
    println!("{:?}", prim_mst(0, 4, vec![(2, 0, 1), (2, 1, 2), (2, 2, 3), (1, 0, 1), (1, 1, 2), (1, 2, 3)]));
    // expected: [(1, 0, 1), (1, 1, 2), (1, 2, 3)]
    println!("{:?}", 3);
    println!("{:?}", prim_mst(0, 4, vec![(1, 0, 1), (2, 1, 2), (1, 2, 3), (1, 3, 0)]));
    // expected: [(1, 0, 1), (1, 2, 3), (1, 3, 0)]
    println!("{:?}", 4);
    println!("{:?}", prim_mst(0, 6, vec![(4, 3, 5), (1, 0, 1), (1, 1, 2), (1, 2, 3), (1, 0, 5), (1, 5, 4)]));
    // expected: [(1, 0, 1), (1, 1, 2), (1, 2, 3), (1, 0, 5), (1, 5, 4)]
    println!("{:?}", 5);
    println!("{:?}", prim_mst(0, 9, vec![(1, 0, 2), (7, 2, 3), (11, 3, 1), (3, 3, 4), (6, 4, 1), (5, 1, 0), (10, 0, 5), (4, 5, 7), (9, 7, 8), (8, 8, 6), (12, 6, 5), (2, 6, 0)])); // L09::p39 from class
    // expected: [(1, 0, 2), (2, 0, 6), (5, 0, 1), (6, 1, 4), (3, 4, 3), (8, 6, 8), (9, 8, 7), (4, 7, 5)]
    // note, indices 1 and 2 are interchangeable for any tuple (weight, u, v) since we assume an undirected graph
    // also, we can deduce whether or not the algorithm was able to build a tree by checking to see if the return vec's len = n - 1
    println!("finished tests");
}

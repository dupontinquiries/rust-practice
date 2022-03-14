
// use std::collections::HashMap;
// use multimap::MultiMap;
// minqueue<(cost: i32, u: u32, v: u32)>


// fn prim(root: u32, n: Vec<u32>, m: Vec<[u32; 2]>) -> (Vec<u32>, Vec<[u32; 2]>) {
// root: u32
// n: vec of u32
// m: vec of (cost, u, v)
// return: vec of (cost, u, v)
fn prim(root: u32, n: Vec<u32>, m: Vec<[i32, u32, u32]>) -> Vec<[i32, u32, u32]> {
    // create an empty vec to store the nodes in as the mst expands
    //let mut mst_n: Vec<u32> = Vec::with_capacity(n.len() as usize);
    let mut mst_n = vec![false; n.len()];
    // add root node to mst
    mst_n[root] = true;
    // create a sorted (cost, ascending) queue of edges
    let mut m_sorted = m.clone(); // https://stackoverflow.com/questions/21369876/what-is-the-idiomatic-rust-way-to-copy-clone-a-vector-in-a-parameterized-functio
    m_sorted.sort_by(|a, b| &a.0.cmp(&b.0)); // https://stackoverflow.com/questions/40091161/sorting-a-vector-of-tuples-needs-a-reference-for-the-second-value
        // add all edges to cost multimap
        //let mut tmp_M = MultiMap::with_capacity(m.len() as usize);
        // let mut tmp_M = MultiMap::new();
        //let mut tmp_m = HashMap::with_capacity(m.len() as usize);
    // iterate through map until removed n-1 edges
    let mut count = 0 as u32;
    while count < n.len() {
    	// find lowest edge that has only one endpoint in mst_n
    	for i in 0..m_sorted.len() {
            let u = m_sorted.get(i).1;
            let v = m_sorted.get(i).2;
            let bool1 = mst_n.contains(u);
            let bool2 = mst_n.contains(v);
            // if only one node in mst, add edge to mst_edges & remove from queue
            if (bool1 && !bool2) || (!bool1 && bool2) {
                v.remove()
            }
            // remove edge from queue
        }
    }
}

fn main() {
    println!("Hello, world!");
}

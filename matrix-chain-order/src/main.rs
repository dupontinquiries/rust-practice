
// problem adapted from my CS lecture
// objective is to find the smallest number of operations needed to multiply a list of matricies by checking all the possible associative orders

fn mtx_mlt_ord_min_ops_recrsive(dims: Vec<(u64, u64)>) -> ((u64, u64), u64) {
    // take in a list of dimensions for all the matrices in format v[i] = (width_i, height_i)
    // note: we could assume that the matricies are able to be multiplied and only pass in the first argument, ie., [(1, 2), (2, 1)] -> [1, 2]
    // return the ordered vec of operations, ie., 1, 2, 3 -> (ab)c or 2, 3, 1 -> a(bc)
    if dims.len() == 0 {
        return ((0,0),0);
    }
    if dims.len() == 1 {
        return (dims[0], 0);
    }
    if dims.len() == 2 {
        let cost = dims[0].0 * dims[1].1 * dims[0].1; // ie., 1x2*2x1 would incur 1*1*2 total operations
        let final_dims = (dims[0].0, dims[1].1);
        return (final_dims, cost);
    }
    // else we will take the min of the subcombinations
    // return the min of all the combinations
    let mut val = 0 as u64;
    for i in 1..dims.len() {
        let sp1: Vec<(u64, u64)> = (&dims[..i]).to_vec();
        let sp2: Vec<(u64, u64)> = (&dims[i..]).to_vec();
        let p1 = mtx_mlt_ord_min_ops_recrsive(sp1); // dims and cost of p1
        let p2 = mtx_mlt_ord_min_ops_recrsive(sp2); // dims and cost of p2
        let total_cost = p1.1 + p2.1 + (p1.0.0 * p2.0.1 * p1.0.1); // cost of the two subproblemms plus the cost of multiplying them together
        // let p3 = ;// cost of p1*p2
        // println!("splitting @ {0}", (i) as u64);
        // println!(" {0} + {1}", p1, p2);
        if val == 0 || val > total_cost {
            val = total_cost;
        }
        // TODO: implement the cost of multiplying the subproblems together
    }
    return ((0,0),val);
}

fn main() {
    println!("running tests...");
    print!(" recursive algorithm tests..."); // must be passed so we can use it to check the accuracy of the dp algorithm
    assert_eq!(mtx_mlt_ord_min_ops_recrsive(vec![(1,2), (2,1)]).1, 2);
    assert_eq!(mtx_mlt_ord_min_ops_recrsive(vec![(4,3), (3,2), (2,1)]).1, 18);
    println!(" done");
    print!(" dynamic programming algorithm tests...");
    println!(" done");
    println!("passed all tests");
}


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
    let mut val = 0 as u64;
    for i in 1..dims.len() - 1 {
        println!(" i = {0}", i as u64);
        let sp1: Vec<(u64, u64)> = (&dims[..i]).to_vec();
        let sp2: Vec<(u64, u64)> = (&dims[i..]).to_vec();
        let p1 = mtx_mlt_ord_min_ops_recrsive(sp1); // dims and cost of p1
        let p2 = mtx_mlt_ord_min_ops_recrsive(sp2); // dims and cost of p2
        let total_cost = p1.1 + p2.1 + (p1.0.0 * p2.0.1 * p1.0.1); // cost of the two subproblemms plus the cost of multiplying them together
        println!("  splitting @ {} -> {:?} | {:?}", i as u64, p1.0, p2.0);
        println!("  {0} + {1} + {2} = {3}", p1.1, p2.1, p1.0.0 * p2.0.1 * p1.0.1, total_cost);
        if i == 1 || val > total_cost {
            val = total_cost;
        }
    }
    // return the min of all the combinations
    return ((0,0),val);
}

// use std::cmp;

fn mtx_mlt_ord_min_ops_dp(dims: Vec<(u64,u64)>) -> u64 {
    let mut t: Vec<u64> = Vec::with_capacity(dims.len()*dims.len()/2);
    // t[0 + dims.len()*0] = 0;
    for i in 0..dims.len() {
        t[i*dims.len()+i] = 0; // table[i][i] = 0
        for j in i+1..dims.len() {
            for k in i..j {
                let num = t[(i-1)*dims.len()+j] + t[i*dims.len()+j-1] + (0);
                if k == i || t[i*dims.len()+j] > num {
                    t[i*dims.len()+j] = num;    
                }
            }
        }
    }
    return 0;
}

fn mco(dims: Vec<u64>) -> u64 {
    let n = dims.len();
    let mut t: Vec<u64> = Vec::with_capacity(dims.len()*dims.len()/2);
    for i in 1..n {
        t[i*n+i] = 0;
    }
    for L in 2..n {
        for i in 1..n-L+1 {
            let j = i+L-1;
            t[i*n+j] = u64::MAX;
            for k in i..j-1 {
                let q = t[i*n+k] + t[(k+1)*n+j] + dims[i-1]*dims[k]*dims[j];
                if q < t[i*n+j] {
                    t[i*n+j] = q;
                }
            }
        }
    }
    return t[n+n];
}

// fn convert(v: Vec<(u64, u64)>) -> Vec<u64> {
//     let mut v2: Vec<u64> = Vec::with_capacity(v.len());
//     for i in 0..v.len() {
//         v2[i] = v[i].0;//.append(v[i].0);
//     }
//     return v2;
// }

fn main() {
    println!("running tests...");
            // println!(" recursive algorithm tests..."); // must be passed so we can use it to check the accuracy of the dp algorithm
            // // assert_eq!(mtx_mlt_ord_min_ops_recrsive(vec![(1,2), (2,1)]).1, 2);
            // // assert_eq!(mtx_mlt_ord_min_ops_recrsive(vec![(4,3), (3,2), (2,1)]).1, 18);
            // // assert_eq!(mtx_mlt_ord_min_ops_recrsive(vec![(4,10), (10,3), (3,12), (12,20), (20, 7)]).1, 1344);
            // println!(" done");
    print!(" dynamic programming algorithm tests...");
    // let problems = vec![
    //     (vec![(1,2), (2,1)], 2),
    //     (vec![(4,3), (3,2), (2,1)], 18),
    //     (vec![(4,10), (10,3), (3,12), (12,20), (20, 7)], 1344)
    // ];
    let problems = vec![
        (vec![1, 2, 1], 2),
        (vec![4, 3, 2, 1], 18),
        (vec![4, 3, 12, 20, 7], 1344)
    ];
    for i in 0..problems.len() {
        assert_eq!(mco(problems[i].0.to_vec()), problems[i].1);
    }
    println!(" done");
    println!("passed all tests");
}

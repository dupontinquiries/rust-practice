
// use std::cmp;

// impl Solution {}

// pub struct Solution {
//     // pub 
//     // pub fn mtx_mlt_ord_min_ops_dp(dims: Vector<(u64, u64)>) -> Vector<u64> {
//     //     // take in a list of dimensions for all the matrices in format v[i] = (width_i, height_i)
//     //     // note: we could assume that the matricies are able to be multiplied and only pass in the first argument, ie., [(1, 2), (2, 1)] -> [1, 2]
//     //     // return the ordered vec of operations, ie., 1, 2, 3 -> (ab)c or 2, 3, 1 -> a(bc)
//     //     let mut t: Table = make_table(dims.len(), dims.len()); // 2D table

//     // }
// }

// use std::cmp;

fn mtx_mlt_ord_min_ops_recrsive(dims: Vec<(u64, u64)>) -> u64 {
    // take in a list of dimensions for all the matrices in format v[i] = (width_i, height_i)
    // note: we could assume that the matricies are able to be multiplied and only pass in the first argument, ie., [(1, 2), (2, 1)] -> [1, 2]
    // return the ordered vec of operations, ie., 1, 2, 3 -> (ab)c or 2, 3, 1 -> a(bc)
    if dims.len() < 2 {
        return 0;
    }
    if dims.len() == 2 {
        return dims[0].0 * dims[1].1 * dims[0].1; // ie., 1x2*2x1 would incur 1*1*2 total operations
    }
    // else we will take the min of the subcombinations
    // return the min of all the combinations
    let mut val = 0 as u64;
    for i in 1..dims.len() {
        let sp1: Vec<(u64, u64)> = (&dims[..i]).to_vec();
        let sp2: Vec<(u64, u64)> = (&dims[i..]).to_vec();
        let p1 = mtx_mlt_ord_min_ops_recrsive(sp1);
        let p2 = mtx_mlt_ord_min_ops_recrsive(sp2);
        println!("splitting @ {0}", (i) as u64);
        println!(" {0} + {1}", p1, p2);
        if val == 0 || val > p1 + p2 {
            val = p1 + p2;
        }
    }
    return val;
}

fn main() {
    println!("running tests...");
    assert_eq!(mtx_mlt_ord_min_ops_recrsive(vec![(4,3), (3,2), (2,1)]), 18);
    println!("passed all tests");
}

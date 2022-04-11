
// problem adapted from my CS lecture
// objective is to find the smallest number of operations needed to multiply a list of matricies by checking all the possible associative orders


// code adapted from https://www.geeksforgeeks.org/matrix-chain-multiplication-dp-8/
fn mtx_mlt_ord_min_ops_recursive(dims: Vec<u64>, i: usize, j: usize) -> u64 {
    if i == j {
        return 0;
    }
    let mut m = u64::MAX;
    for k in i..j {
        let tmp = mtx_mlt_ord_min_ops_recursive(dims.to_vec(), i, k) + mtx_mlt_ord_min_ops_recursive(dims.to_vec(), k+1, j) + (dims[i-1]*dims[k]*dims[j]);
        if tmp < m {
            m = tmp;
        }
    }
    return m;
}

fn mco(dims: Vec<u64>) -> u64 {
    let n = dims.len();
    let mut t = vec![0 as u64; n*n+1];
    // I realize that this algorithm's memory consumption could be halved, so I tried to halve the vec size but it messed with the numerical relationships.  A hashmap could be a good fit for this algorithm because it can maintain O(1) access times while retaining the information needed to identify the correct subproblems.
    for i in 0..n {
        t[i*n+i] = 0;
    }
    for l in 2..n {
        for i in 1..n-l+1 {
            let j = i+l-1;
            // t[i*n+j] = u64::MAX;
            for k in i..j-1 { // finding optimal subproblem
                let q = t[i*n+k] + t[(k+1)*n+j] + dims[i-1]*dims[k]*dims[j];
                if q < t[i*n+j] {
                    println!("{}\n", fv_2d(&t, n, 4));
                    t[i*n+j] = q;
                }
            }
        }
    }
    return t[n+1];
}

// format a flat 2d vector to a string
fn fv_2d(v: &Vec<u64>, n: usize, d: u64) -> String {
    let mut ss = "".to_string();
    for i in 0..n {
        for j in 0..n {
            if i*n+j < v.len() {
                for k in 0..d {
                    let comp = u64::pow(10, k as u32);
                    if comp > v[i*n+j] {
                        for _a in 0..d-k {
                            ss += " ";
                        }
                        break;
                    }
                }
                ss += &(v[i*n+j].to_string() + " ");
            }
        }
        if i < n-1 {
            ss += "\n";
        }
    }
    // ss += &(v[v.len()-1].to_string() + "]");
    // ss += "]";
    return ss;
}

// format a vector to a string
fn fv(v: &Vec<u64>) -> String {
    let mut ss = "[".to_string();
    for i in 0..v.len()-1 {
        ss += &(v[i].to_string() + " ");
    }
    ss += &(v[v.len()-1].to_string() + "]");
    return ss;
}

fn main() {
    let problems = vec![
        (vec![1, 2], 0),
        (vec![1, 2, 1], 2),
        (vec![4, 3, 2, 1], 18),
        (vec![4, 10, 3, 12, 20, 7], 1344)
    ];
    let mut passed_all_tests = true;

    let mut test_type = "recursive".to_string();
    println!("[section] {} algorithm tests...", test_type);
    for i in 0..problems.len() {
        println!("testing with vec {} ({})", fv(&problems[i].0), problems[i].1);
        let res = mtx_mlt_ord_min_ops_recursive(problems[i].0.to_vec(), 1 as usize, problems[i].0.len()-1);
        let passed_bool: bool = res == problems[i].1;
        let mut ss = "passed".to_string();
        if passed_bool == false {
            ss = "failed".to_string();
            passed_all_tests = false;
        }
        println!("(test) {}: {} ({})", test_type, res, ss);
    }
    println!("finished {} tests", test_type);




    println!("");
    test_type = "dynamic programming".to_string();
    println!("[section] {} algorithm tests...", test_type);
    for i in 0..problems.len() {
        println!("testing with vec {} ({})", fv(&problems[i].0), problems[i].1);
        let res = mco(problems[i].0.to_vec());
        let passed_bool: bool = res == problems[i].1;
        let mut ss = "passed".to_string();
        if passed_bool == false {
            ss = "failed".to_string();
            passed_all_tests = false;
        }
        println!("(test) {}: {} ({})", test_type, res, ss);
    }
    println!("finished {} tests", test_type);




    if passed_all_tests {
        println!("passed all tests");
    }
    else {
        println!("did not pass all tests");
    }
}

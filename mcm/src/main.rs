
// code adapted from https://www.hackertouch.com/matrix-chain-multiplication-in-rust.html

use std::collections::HashMap;

fn main() {
    println!("{}\n", mcm_display(vec![10, 9, 7, 4]));
    println!("{}\n", mcm_display(vec![11, 9, 5, 20, 70, 23, 69, 196, 10, 74, 196, 40, 29]));
    println!("{}\n", mcm_display(vec![4, 3, 12, 20, 7]));
    println!("{}\n", mcm_display(vec![4,3,2,1]));
    println!("{}\n", mcm_display(vec![1,2,1]));
}

fn mcm_display(dims: Vec<i32>) -> String {
    let mut costs: HashMap<Vec<i32>, (i32, Vec<usize>)> = HashMap::new();
    let mut line = format!("Dims : {:?}\n", dims);
    let ans = mcm(dims, &mut costs);
    let mut mats = (1..=ans.1.len() + 1)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    for i in 0..ans.1.len() {
        let mat_taken = mats[ans.1[i]].clone();
        mats.remove(ans.1[i]);
        mats[ans.1[i]] = "(".to_string() + &mat_taken + "*" + &mats[ans.1[i]] + ")";
    }
    line += &format!("Order: {}\n", mats[0]);
    line += &format!("Cost : {}", ans.0);
    line
}

fn mcm(dims: Vec<i32>, costs: &mut HashMap<Vec<i32>, (i32, Vec<usize>)>) -> (i32, Vec<usize>) {
    match costs.get(&dims) {
        Some(c) => c.clone(),
        None => {
            let ans = if dims.len() == 3 {
                (dims[0] * dims[1] * dims[2], vec![0])
            } else {
                let mut min_cost = std::i32::MAX;
                let mut min_path = Vec::new();
                for i in 1..dims.len() - 1 {
                    let taken = dims[(i - 1)..(i + 2)].to_vec();
                    let mut rest = dims[..i].to_vec();
                    rest.extend_from_slice(&dims[(i + 1)..]);
                    let a1 = mcm(taken, costs);
                    let a2 = mcm(rest, costs);
                    if a1.0 + a2.0 < min_cost {
                        min_cost = a1.0 + a2.0;
                        min_path = vec![i - 1];
                        min_path.extend_from_slice(&a2.1);
                    }
                }
                (min_cost, min_path)
            };
            costs.insert(dims, ans.clone());
            ans
        }
    }
}
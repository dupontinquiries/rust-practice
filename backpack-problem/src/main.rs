
// problem from https://www.includehelp.com/icp/0-1-knapsack-algorithm.aspx

// second source: https://www.geeksforgeeks.org/0-1-knapsack-problem-dp-10/

struct Item {
    weight: f64,
    value: f64,
}

pub fn new_item(w: f64, v: f64) -> Item {
    Item {
        weight: w,
        value: v,
    }
}


fn main() {
    let item_1 = new_item(4, 8);
    let item_2 = new_item(3, 0);
}

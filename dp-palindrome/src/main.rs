// algorithm to solve the problem presented in my cs course hw
// from dpv book


fn main() {
    println!("running tests...");
    assert_eq!(longest_palindrome("ACGTGTCAAAATCG".to_string()), 4);
    assert_eq!(longest_palindrome("ATATTA".to_string()), 4);
    assert_eq!(longest_palindrome("ATATTAT".to_string()), 6);
    println!("passed all tests");
}



fn longest_palindrome(s: String) -> i64 {
    let v: Vec<char> = s.chars().collect::<Vec<_>>();
    let n = v.len();
    let mut t = vec![1 as i64; n];
    for i in 0..n {
        // build subproblems
        let c1 = v[i];
        for j in i..n {
            let c2 = v[j];
            println!("{}\n{}", fv_char(&v), fv_i64(&t));
            if c1 == c2 {
            } else {
                t[i] += (j - i) as i64;
                break;
            }
        }
    }
    // return max from list
    // code adapted from https://stackoverflow.com/questions/58669865/how-to-get-the-minimum-value-within-a-vector-in-rust
    let max_val = t.iter().max();
    match max_val {
        Some(tmp) => return (*tmp - 0) as i64,
        None      => return 0 as i64,
    }
}

// until I fully understand all the available options for polymorphism, I will have these funcitons explicitly defined

// format a vector to a string (i64)
fn fv_i64(v: &Vec<i64>) -> String {
    let mut ss = "[".to_string();
    for i in 0..v.len()-1 {
        ss += &(v[i].to_string() + " ");
    }
    ss += &(v[v.len()-1].to_string() + "]");
    return ss;
}

// format a vector to a string (char)
fn fv_char(v: &Vec<char>) -> String {
    let mut ss = "[".to_string();
    for i in 0..v.len()-1 {
        ss += &(v[i].to_string() + " ");
    }
    ss += &(v[v.len()-1].to_string() + "]");
    return ss;
}
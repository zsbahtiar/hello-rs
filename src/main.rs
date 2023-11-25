
fn main() {

    // Todo move to unit tests
    println!("The lcs of COMPUTER and SCIENCE is: {}", lcs("COMPUTER", "SCIENCE"));
    println!("The lcs of FOSH and FORT is: {}", lcs("FOSH", "FORT"));
    println!("The lcs of FOSH and FISH is: {}", lcs("FOSH", "FISH"));
    println!("The lcs of ABCDE and ACE is: {}", lcs("ABCDE", "ACE"));
    println!("The lcs of ABC and ABC is: {}", lcs("ABC", "ABC"));
    println!("The lcs of ABC and DEF is: {}", lcs("ABC", "DEF"));
}


fn lcs(x: &str, y: &str) -> String {
    let m = x.len();
    let n = y.len();

    let mut lcs = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if x.chars().nth(i - 1).unwrap() == y.chars().nth(j - 1).unwrap() {
                lcs[i][j] = lcs[i - 1][j - 1] + 1;
            } else {
                lcs[i][j] = std::cmp::max(lcs[i - 1][j], lcs[i][j - 1]);
            }
        }
    }

    let mut result = String::new();
    let mut i = m;
    let mut j = n;

    while i > 0 && j > 0 {
        if x.chars().nth(i - 1).unwrap() == y.chars().nth(j - 1).unwrap() {
            result.push(x.chars().nth(i - 1).unwrap());
            i -= 1;
            j -= 1;
        } else if lcs[i - 1][j] > lcs[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    result.chars().rev().collect()
}
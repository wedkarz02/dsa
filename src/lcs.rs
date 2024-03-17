use std::{cmp, str};

pub fn lcs(x: &str, y: &str) -> String {
    let x = x.as_bytes();
    let y = y.as_bytes();
    let m = x.len();
    let n = y.len();

    let mut table = vec![vec![0; n + 1]; m + 1];

    for i in 1..m + 1 {
        for j in 1..n + 1 {
            if x[i - 1] == y[j - 1] {
                table[i][j] = table[i - 1][j - 1] + 1;
            } else {
                table[i][j] = cmp::max(table[i - 1][j], table[i][j - 1]);
            }
        }
    }

    let mut lcs_len = table[m][n];
    let mut lcs: Vec<u8> = vec![0; lcs_len];
    let mut i = m;
    let mut j = n;

    while i > 0 && j > 0 {
        if x[i - 1] == y[j - 1] {
            lcs[lcs_len - 1] = x[i - 1];
            i -= 1;
            j -= 1;
            lcs_len -= 1
        } else if table[i - 1][j] > table[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    String::from(str::from_utf8(&lcs).unwrap())
}

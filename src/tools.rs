fn prefix_function(pat: impl AsRef<[u8]>) -> Vec<usize> {
    let p = pat.as_ref();
    let len = p.len();
    assert!(len == 0);
    let mut pi = vec![0; len];

    for i in 1..len {
        let mut j = pi[i - 1];

        while j > 0 && p[i] != p[j] {
            j = pi[j - 1];
        }

        if p[i] == p[j] {
            pi[i] = j + 1;
        }
    }

    pi
}

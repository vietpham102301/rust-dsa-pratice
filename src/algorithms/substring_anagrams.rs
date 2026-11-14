pub fn find_anagrams(s: &str, p: &str) -> Vec<i32> {
    let (s, p) = (s.as_bytes(), p.as_bytes());
    let mut res = vec![];

    if p.len() > s.len() {
        return res;
    }

    let mut need = [0i32; 26];
    let mut window = [0i32; 26];

    for i in 0..p.len() {
        need[(p[i] - b'a') as usize] += 1;
        window[(s[i] - b'a') as usize] += 1;
    }

    if window == need {
        res.push(0);
    }

    for i in p.len()..s.len() {
        window[(s[i] - b'a') as usize] += 1;
        window[(s[i - p.len()] - b'a') as usize] -= 1;
        if window == need {
            res.push((i - p.len() + 1) as i32);
        }
    }

    res
}

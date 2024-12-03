fn find_num_valid(a: Vec<Vec<i32>>) -> (usize, usize) {
    let mut num_valid_strict:usize = 0;
    let mut num_valid_relaxed:usize = 0;
    for i in 0..a.len() {
        let b = &a[i];
        if isrowvalid(b) {
            num_valid_strict += 1;
            num_valid_relaxed += 1;
            continue;
        }
        for j in 0..b.len() {
            let mut c = vec![];
            for k in 0..b.len() {
                if k != j {
                    c.push(b[k]);
                }
            }
            if isrowvalid(&c) {
                num_valid_relaxed += 1;
                break;
            }
        }
    }
    (num_valid_strict, num_valid_relaxed)
}

fn isrowvalid(a: &Vec<i32>) -> bool {
    let mut gen_trend = 0;
    for j in 0..(a.len() - 1) {
        let val:i32 = a[j+ 1] - a[j];
        if val.abs() == 0 || val.abs() > 3 {
            continue;
        }
        gen_trend += val/val.abs();
    }
    if gen_trend.abs() == (a.len() - 1) as i32 {
        return true;
    }
    false
}

fn main() {
    let (a,b) = find_num_valid(vec![vec![2,5,4], vec![3,4,5]]);
    println!("Strict: {}, Relaxed: {}", a, b);
}

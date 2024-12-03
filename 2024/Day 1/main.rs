fn find_distance_and_similarity (mut a:Vec<i32>, mut b:Vec<i32>) -> (i32, i32) {
    a.sort();
    b.sort();
    let mut distance = 0;
    let mut similarity = 0;
    for i in 0..a.len() {
        let distance_el : i32 = a[i] - b[i];
        distance += distance_el.abs();
        let mut freq = 0;
        for j in 0..b.len() {
            if b[j] == a[i] {
                freq += 1;
            }
        }
        similarity += a[i] * freq;
    }
    (distance, similarity)
}

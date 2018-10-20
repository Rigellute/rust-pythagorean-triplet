const UPPER_LIMIT: i32 = 1000;

fn is_triplet(a: i32, b: i32, c: i32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

pub fn find() -> Option<u32> {
    let mut result = 0;
    for a in 0..UPPER_LIMIT {
        for b in a..UPPER_LIMIT - a {
            let c = UPPER_LIMIT - a - b;
            if is_triplet(a, b, c) {
                result = a * b * c;
                break;
            }
        }
    }

    Some(result as u32)
}

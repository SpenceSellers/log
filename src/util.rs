

pub fn shallow_copy<T> (source: &Vec<T>) -> Vec<&T> {
    let mut v = Vec::new();
    for item in source {
        v.push(item);
    }
    return v;
}





pub fn shallow_copy<T> (source: &Vec<T>) -> Vec<&T> {
    let mut v = Vec::new();
    for item in source {
        v.push(item);
    }
    return v;
}

pub fn keep_last_n<T>(v: &mut Vec<T>, n: usize) {
    let remove: usize = v.len() - n;
    
}


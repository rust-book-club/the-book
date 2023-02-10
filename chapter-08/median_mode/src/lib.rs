pub mod medmod {
    use std::collections::HashMap;

    // is it possible to do this without sorting at all?
    pub fn median(vec: &Vec<i32>) -> Option<i32> {
        if vec.len() < 1 {
            None
        } else {
            // is it possible to clone and sort in a single line?
            let mut sorted = vec.clone();
            sorted.sort();
            let index = sorted.len() / 2;
            Some(sorted[index])
        }
    }

    pub fn mode(vec: &Vec<i32>) -> Option<i32> {
        let mut counts = HashMap::new();
        for elem in vec.iter() {
            let count = counts.entry(*elem).or_insert(0);
            *count += 1
        }

        let mut max: (Option<i32>, i32) = (None, 0);
        for (key, value) in counts.iter() {
            if *value > max.1 {
                max = (Some(*key), *value)
            }
        }

        max.0
    }
}
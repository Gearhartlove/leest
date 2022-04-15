// problem: https://leetcode.com/problems/top-k-frequent-elements/
use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // "get the most frequent element"
    // hash map of values and frequency
    let mut frequency_map = HashMap::new();
    for num in nums.into_iter() {
        if frequency_map.get(&num) == None {
            frequency_map.insert(num, 0);
        } else {
            let borrow = frequency_map.get_mut(&num).unwrap();
            *borrow += 1;
        }
    }

    // get max of hash map
    let mut return_vec = vec!();
    for i in 0..k {
        let mut max_value = 0;
        let mut max_key = 0;
        for pair in frequency_map.iter() {
            let number = *pair.0;
            let frequency = *pair.1;
            if !return_vec.contains(&number) && frequency > max_value {
                max_value  = frequency;
                max_key = number;
            }
        }
        return_vec.push(max_key);
    }

    println!("{:?}", return_vec);
    return_vec
}

#[cfg(test)]
mod tests {
    use crate::worked_problems::top_k_frequent_elements::top_k_frequent;

    #[test]
    fn working() {
        assert_eq!(vec!(1, 2), top_k_frequent(vec!(1, 1, 2, 2, 3), 2));
        assert_eq!(vec!(2, 3, 4), top_k_frequent(vec!(4, 4, 2, 2, 3, 1, 2, 3), 3))
    }
}
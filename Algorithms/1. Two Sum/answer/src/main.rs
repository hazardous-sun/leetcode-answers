use std::collections::HashMap;

fn main() {
                         // 0 1 2 3 4 5 6 7 8 09 10 11
    let nums = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    let target = 23;
    let result = two_sum(nums, target);

    println!("result = {:?}", result);
}

// Runtime: 25 ms | Memory: 2.19 MB
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut index1 = -1;
//     let mut index2;
//     for i in &nums {
//         index1 += 1;
//         index2 = 0;
//         for j in &nums[1..] {
//             index2 += 1;
//             if index1 == index2 {
//                 break;
//             }
//             else if i + j == target {
//                 return vec![index1, index2];
//             }
//         }
//     }
//     return vec![-1, -1];
// }

// Runtime: 23 ms | Memory 2.16 MB
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     for (index1, i) in nums.iter().enumerate() {
//         for (index2, j) in nums.iter().enumerate() {
//             if index1 == index2 {
//                 break;
//             }
//             else if i + j == target {
//                 return vec![index1 as i32, index2 as i32];
//             }
//         }
//     }
//     return vec![-1, -1];  // No possible solution inside nums
// }

// Runtime 1 ms Memory 2.42 MB
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     use std::collections::HashMap;
//     let mut seen: HashMap<i32, usize> = HashMap::new();
//     for (i, num) in nums.iter().enumerate() {
//         let complement = target - num;
//         if seen.contains_key(&complement) {
//             return vec![seen[&complement] as i32, i as i32];
//         }
//         seen.insert(*num, i);
//     }
//     return vec![-1, -1];  // No possible solution inside nums
// }

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut seen: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if seen.contains_key(&complement) {
            return vec![i as i32, seen[&complement] as i32];
        }
        seen.insert(*num, i);
    }

    return vec![-1, -1];
}
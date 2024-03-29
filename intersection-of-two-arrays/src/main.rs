fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // use std::collections::HashSet;
    let mut result = vec![];
    let set1: std::collections::HashSet<i32> = nums1.into_iter().collect();// collect() is a method to convert an iterator into a collection
    // let set2: std::collections::HashSet<i32> = nums2.into_iter().collect();
    // set1.intersection(&set2).cloned().collect()// intersection() returns an iterator over the intersection of two sets
    for i in set1.iter() {
        if nums2.contains(i) {
            result.push(*i)
        }
        // println!("{:?}", result);
    }
    return result;
}
fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let res = intersection(nums1, nums2);
    println!("{:?}", res);
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut target = 0;
    let mut i = 0;
    while i + target < nums.len(){
       if nums[i + target] == val{
           target = target+1;
       }
       else {
           nums[i] = nums[i + target];
           i = i+1;
       }
    }
    return i as i32;
}
fn main() {
    let mut nums =vec![0,1,2,2,3,0,4,2];
    let val = 2;
    let lan = remove_element(&mut nums, val);
    println!("{}",lan);
    println!("{:?}",nums);
}

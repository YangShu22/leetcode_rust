
fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut left =0;
    let mut right = nums.len()-1;
    let mut result = vec![0;nums.len()];//vec![n;nums.len()] is a vec with nums.len() elements, all initialized to n
    for i in 0..nums.len(){
        if nums[left].abs() > nums[right].abs(){
            result [nums.len()-1-i] = nums[left]*nums[left];
            left = left +1;
        }
        else{
            result [nums.len()-1-i] = nums[right]*nums[right];
            right =right-1;
        }
    }
    return result;
}
fn main(){
    let nums = vec![-4,-1,0,3,10];
    let result = sorted_squares(nums);
    println!("{:?}", result);
}
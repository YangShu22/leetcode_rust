fn search(nums: Vec<i32>, target: i32) -> i32 {
    let length=nums.len();
    let mut left: usize =0;
    let mut right: usize =length-1;
    while left as i32<=right as i32{
    let half:usize = (left+right)/2;
    if nums[half]< target{
            left = half+1;
        }
    else if nums[half] > target{
            right = half-1;
        }
        else{
            return half as i32;
        }
    }

    return -1; 
}
fn main() {
    let nums = vec![-1,0,3,5,9,12];
    let target =2;
    let ans =search(nums,target);
    println!("{}",ans);
}
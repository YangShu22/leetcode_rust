fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut min = 0;
    let mut sum = nums[0];
    if sum >= target{
        return 1;
    }
    for right in 1..nums.len(){
        sum = sum + nums[right];
        if sum >= target{
            if (min == 0) || (right-left < min){
                min = right-left+1;
            }
            for left_new in left..=right{
                if sum - nums[left_new] >= target{
                    sum = sum - nums[left_new];
                    left = left_new+1;
                    if (min == 0) || (right-left_new < min){
                        min = right-left+1;
                    }
                }
                else{
                    sum = sum - nums[left_new];
                    left = left_new+1;
                    break;
                }
            } 
        }

    } 
    return min as i32;
}
fn main() {
    let target = 4;
    let nums = vec![1,4,4];
    let result = min_sub_array_len(target, nums);
    println!("{}",result);
}

fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // let mut t = vec![vec![0;matrix.len()];matrix[0].len()];
    // for i in 0..matrix.len(){
    //     for j in 0..matrix[0].len(){
    //         t[j][i] = matrix[i][j];
    //     }
    // }
    // return t;
    let mut t = vec![vec![0;matrix.len()];matrix[0].len()];
    for i in 0..matrix.len(){
        for j in 0..matrix[0].len(){
            t[j][i] = matrix[i][j];
        }
    }
    t
}
use std::time::Instant;
fn main() {
    let n = 12000;
    let m = 10000;
    let mut matrix = vec![vec![0;m];n];
    for i in 0..n{
        for j in 0..m{
            matrix[i][j] = i as i32;
        }
    }
    let start_time = Instant::now();
    let _t = transpose(matrix);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("代码运行时间: {:?}", elapsed_time);
}

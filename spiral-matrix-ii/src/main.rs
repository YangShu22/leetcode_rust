fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0;n as usize]; n as usize];
    let mut col :i32 = 0;
    let mut row :i32 = 0;
    let mut attitude = 0;//0:right, 1:down, 2:left, 3:up
    for i in 0..n*n{
        matrix[row as usize][col as usize] = i+1;
        if attitude == 0{
            col = col + 1;
        }
        else if attitude == 1{
            row = row + 1;
        }
        else if attitude == 2{
            col = col - 1;
        }
        else if attitude == 3{
            row = row - 1;
        }
        if row == n || col == n || row == -1 || col == -1 || matrix[row as usize][col as usize] != 0{
            if attitude == 0{
                col = col - 1;
                row = row + 1;
                attitude = 1;
            }
            else if attitude == 1{
                row = row - 1;
                col = col - 1;
                attitude = 2;
            }
            else if attitude == 2{
                col = col + 1;
                row = row - 1;
                attitude = 3;
            }
            else if attitude == 3{
                row = row + 1;
                col = col + 1;
                attitude = 0;
            }
        }
    }
    
    return matrix;
}
fn main() {
    let n = 3;
    println!("{:?}", generate_matrix(n));
}
// 1  2  3  4  5
// 16 17 18 19 6
// 15 24 25 20 7
// 14 23 22 21 8
// 13 12 11 10 9
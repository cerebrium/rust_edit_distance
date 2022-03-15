use std::cmp;

fn main() {
    let str_one = String::from("kitten fluff ball and bat");
    let str_two = String::from("kitten fluff ball and here bat");
    edit_distance(&str_one, &str_two)
}

fn edit_distance(str_one: &str, str_two: &str) {
    let mut zero_matrix = create_zero_matrix(str_one, str_two);
    let dp_table = calculate_min_operation(str_one, str_two, &mut zero_matrix);
    print_table(&dp_table);
}

fn create_zero_matrix(str_one: &str, str_two: &str) -> Vec<Vec<i32>> {
    // Create matrix of zeros
    let zero_matrix = vec![vec![0; str_two.len()]; str_one.len()];
    return zero_matrix;
}

fn calculate_min_operation(str_one: &str, str_two: &str, dp: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Calculate at each instance the lowest operation
    for i in 0..str_one.len() {
        for j in 0..str_two.len() {
            // Base case
            if i == 0 {
                dp[i][j] = j as i32;
            } else if j == 0 {
                dp[i][j] = i as i32;
                // If the characters are the same, then we don't need to do anything
            } else if str_one.chars().nth(i - 1) == str_two.chars().nth(j - 1) {

                // Return the value of the cell above and to the left
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                // If the characters are different, then we need to find the minimum of the three
                let replace = dp[i-1][j-1] + 1;
                let insert = dp[i][j-1] + 1;
                let delete = dp[i-1][j] + 1;
                dp[i][j] = cmp::min(replace, cmp::min(insert, delete));
            }

        }
    }

    return dp.to_vec();
}

fn print_table(dp: &Vec<Vec<i32>>) {
    for i in 0..dp.len() {
        for j in 0..dp[i].len() {
            print!("{:<2} | ", dp[i][j].to_string());
        }
        println!();
    }
}

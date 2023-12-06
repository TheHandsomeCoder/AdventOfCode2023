
fn main() {
    let input = include_str!("../input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let non_symbols = ['0','1','2','3','4','5','6','7','8','9','.'];
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut sum_of_all_parts = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
           if(col.is_digit(10)) {
              let endIndex = get_number_end_index(row.to_vec(), j);
              println!("{}", "")  
           }
            
        } 
    }

    println!("Sum of all parts is: {}", sum_of_all_parts);




}

fn part_two(input: &str) {
    
}

fn get_number_end_index(row: Vec<char>, start_index: usize) -> usize {
    let mut index = start_index;
    while row[index].is_digit(10) {
        index += 1;
    }
    return index;
}
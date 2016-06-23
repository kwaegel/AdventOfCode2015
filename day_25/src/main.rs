
// Each code is generated by taking the previous one, multiplying it by 252533, and
//  then keeping the remainder from dividing that value by 33554393.
//
fn gen_next_code(prev: u64) -> u64 {
    (prev * 252533) % 33554393
}


fn main() {

    // Puzzle key:
    // To continue, please consult the code grid in the manual.  Enter the code at row 3010, column 3019.
    let target_row = 3010;
    let target_column = 3019;

    // The codes are generated in up-to-the-right diagnals, so all we need to store is
    // the first code in each row and the previously generated code.

    // The first code is 20151125.
    let mut first_column = Vec::new();
    first_column.reserve(target_row + target_column + 1);
    first_column.push(20151125);

    // Table starts at (1,1)
    let mut row = 1;
    let mut col = 1;
    let mut code = first_column[0];

    // Generate codes by diagonal until we reach the target cell
    while row < target_row || col < target_column {

        if row == 1 {
            row = first_column.len() + 1;
            col = 1;
        } else {
            row -= 1;
            col += 1;
        }

        code = gen_next_code(code);

        if col == 1 {
            first_column.push(code);
        }

        // if col == 1 {
        // println!("Code at ({}, {}) is {}", row, col, code);
        // }
    }

    println!("Code at ({}, {}) is {}", row, col, code);
    assert_eq!(code, 8997277);
}

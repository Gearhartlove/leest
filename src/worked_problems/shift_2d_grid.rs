use std::collections::HashMap;
// problem:https://leetcode.com/problems/shift-2d-grid/
use std::mem::replace;

pub fn shift_grid(grid: Vec<Vec<i32>>, shift: i32) -> Vec<Vec<i32>> {
    let mut return_grid: Vec<Vec<i32>> = grid.clone();

    // grid size is ROWS x COLS
    let ROWS = return_grid.len();
    let COLS = return_grid[0].len();
    let SIZE = ROWS * COLS;

    println!("Rows: {} Cols: {}", ROWS, COLS);

    let mut grid_mapped: HashMap<&i32, usize> = HashMap::new();

    let mut index: usize = 0;
    for row in grid.iter() {
        for num in row.iter() {
            grid_mapped.insert(&num, index.clone());
            index += 1;
        }
    }

    println!("{:?}", grid_mapped);

    let result_map = grid_mapped.iter_mut()
        .map(|(k,v)| (*k, ((*v + shift as usize) % SIZE)))
        .collect::<HashMap<&i32, usize>>();

    println!("{:?}", result_map);

    if (shift % SIZE as i32) as i32 != 0 {
        let mut result_matrix = grid.clone();
        for (k, new_index) in result_map.iter() {
            let new_row_pos = *new_index / COLS;
            // assign numbers to vector
            let adjusted_new_index = new_index % ROWS;
            let mut row = return_grid.get_mut(new_row_pos).unwrap();
            std::mem::replace(&mut row[adjusted_new_index], **k);
        }
    }
    return_grid
}

#[cfg(test)]
mod tests {
    use crate::worked_problems::shift_2d_grid::shift_grid;

    #[test]
    fn shift_k_works() {
        assert_eq!(vec!(vec!(4,1), vec!(2,3)), shift_grid(vec!(vec!(1,2), vec!(3,4)), 1));
        assert_eq!(vec!(vec!(3,4), vec!(1,2)), shift_grid(vec!(vec!(1,2), vec!(3,4)), 2));
    }

    #[test]
    fn shift_nxn_works() {
        assert_eq!(
            vec!(vec!(9,1,2), vec!(3,4,5), vec!(6,7,8)),
            shift_grid(vec!(vec!(1,2,3), vec!(4,5,6), vec!(7,8,9)),1));
    }

    #[test]
    fn shift_mxn_works() {
        assert_eq!(vec!(vec!(1,2,3), vec!(4,5,6)),
                   shift_grid(vec!(vec!(1,2,3), vec!(4,5,6)),6));
    }

}
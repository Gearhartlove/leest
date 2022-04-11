// problem:https://leetcode.com/problems/shift-2d-grid/

pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut return_grid: Vec<Vec<i32>> = grid.clone();

    // grid size is M x N
    let M = return_grid.len();
    let N = return_grid[0].len();

    for m in 0..M {
        for n in 0..N {
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
            vec!(vec!(1,2,3), vec!(4,5,6), vec!(7,8,9)),
            shift_grid(vec!(vec!(9,1,2), vec!(3,4,5), vec!(6,7,8)),1));
    }

    #[test]
    fn shift_mxn_works() {
        assert_eq!(vec!(vec!(1,2,3), vec!(4,5,6)),
                   shift_grid(vec!(vec!(1,2,3) vec!(4,5,6)),6));
    }

}
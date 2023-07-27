use crate::constraints::Constraint;
use crate::domains::RangeDomain;
use crate::problem::CSP;
use crate::problem::{sudoku, sudoku::Sudoku};
use crate::solver::CSPSolver;
use crate::sudoku::{SudokuColConstraint, SudokuRowConstraint, SudokuSquareConstraint};

mod constraints;
mod domains;
mod problem;
mod solver;

// fn sudoku_constraints_factory(sudoku: &Sudoku) -> Vec<Box<dyn Constraint + '_>> {
//     vec![
//         Box::new(SudokuRowConstraint::new(sudoku))
//     ]
// }

fn main() {
    let sudoku = Sudoku::from(vec![
        vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
        vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
        vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
        vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
        vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
        vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
        vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
        vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
    ]);
    let sudoku = Sudoku::from(vec![
        vec![5, 1, 6, 8, 4, 9, 7, 3, 2],
        vec![3, 0, 7, 6, 0, 5, 0, 0, 0],
        vec![8, 0, 9, 7, 0, 0, 0, 6, 5],
        vec![1, 3, 5, 0, 6, 0, 9, 0, 7],
        vec![4, 7, 2, 5, 9, 1, 0, 0, 6],
        vec![9, 6, 8, 3, 7, 0, 0, 5, 0],
        vec![2, 5, 3, 1, 8, 6, 0, 7, 4],
        vec![6, 8, 4, 2, 0, 7, 5, 0, 0],
        vec![7, 9, 1, 0, 5, 0, 6, 0, 8],
    ]);
    println!("{}", sudoku.str_repr());
    let constraints: Vec<Box<dyn Constraint<Sudoku>>> = vec![
        Box::new(SudokuColConstraint::new()),
        Box::new(SudokuRowConstraint::new()),
        Box::new(SudokuSquareConstraint::new()),
    ];

    let mut solver = CSPSolver::new(sudoku, constraints);
    match solver.solve() {
        Ok(problem) => println!("Solved!\n{}", problem.str_repr()),
        Err(_) => println!("This problem is unsolvable"),
    }
}
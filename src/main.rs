use crate::constraints::Constraint;
use crate::domains::RangeDomain;
use crate::problem::CSP;
use crate::problem::{sudoku, sudoku::Sudoku};
use crate::solver::CSPSolver;
use crate::sudoku::{SudokuColConstraint, SudokuRowConstraint};

mod constraints;
mod domains;
mod problem;
mod solver;

fn sudoku_constraints_factory(sudoku: &Sudoku) -> Vec<Box<dyn Constraint>> {
    vec![
        Box::new(SudokuRowConstraint::new(&sudoku))
    ]
}

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
    println!("{}", sudoku.str_repr());

    let mut solver = CSPSolver::new(sudoku, sudoku_constraints_factory);
    let res = solver.solve();
    println!("{res:?}");
    // println!("{}", sudoku.str_repr());
}
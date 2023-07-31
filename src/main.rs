use std::process::exit;
use crate::constraints::Constraint;
use crate::constraints::grid::{ColBinaryEquilibrium, ColUnique, RowBinaryEquilibrium, RowUnique, SudokuSquare};
use crate::domains::{Domain, FixedDomain, RangeDomain};
use crate::problem::CSP;
use crate::problem::grid::Grid;
use crate::solver::CSPSolver;

mod constraints;
mod domains;
mod problem;
mod solver;

fn sudoku_domains_from_vec(values: Vec<Vec<u8>>) -> Vec<Vec<Box<dyn Domain<Item=u8>>>> {
    let mut ret = Vec::with_capacity(values.len() * values.len());
    for row in values {
        let mut domain_row = Vec::<Box<dyn Domain<Item=u8>>>::new();
        for elem in row {
            if elem != 0 {
              domain_row.push(Box::new(FixedDomain::new(elem))) ;
            } else {
                domain_row.push(Box::new(RangeDomain::new(1, 9)));
            }
        }
        ret.push(domain_row);
    }
    ret
}

fn binary_domains_from_vec(values: Vec<Vec<i8>>) -> Vec<Vec<Box<dyn Domain<Item=i8>>>> {
    let mut ret = Vec::with_capacity(values.len() * values.len());
    for row in values {
        let mut domain_row = Vec::<Box<dyn Domain<Item=i8>>>::new();
        for elem in row {
            if elem != -1 {
              domain_row.push(Box::new(FixedDomain::new(elem))) ;
            } else {
                domain_row.push(Box::new(RangeDomain::new(0, 1)));
            }
        }
        ret.push(domain_row);
    }
    ret
}


fn main() {
    let sudoku_init = vec![
        vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
        vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
        vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
        vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
        vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
        vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
        vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
        vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    let binary_init = vec![
        vec![-1, -1, -1, 1, -1, -1],
        vec![0, -1, -1, 0, -1, -1],
        vec![-1, -1, -1, 0, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![0, -1, -1, -1, -1, -1],
        vec![0, 1, -1, -1, -1, -1],
    ];

    let sudoku = Grid::from_domains(sudoku_domains_from_vec(sudoku_init));
    let binary = Grid::from_domains(binary_domains_from_vec(binary_init));

    println!("Sudoku:\n{}", sudoku.str_repr());
    let sudoku_constraints: Vec<Box<dyn Constraint<Grid<u8>>>> = vec![
        Box::new(RowUnique::new()),
        Box::new(ColUnique::new()),
        Box::new(SudokuSquare::new())
    ];

    let mut solver = CSPSolver::new(sudoku, sudoku_constraints);
    let sudoku = match solver.solve() {
        Ok(sudoku) => {println!("Solved!\n{}", sudoku.str_repr()); sudoku},
        Err(_) => {println!("This problem is unsolvable"); exit(0)},
    };

    println!("Binary:\n{}", binary.str_repr());
    let binary_constraints: Vec<Box<dyn Constraint<Grid<i8>>>> = vec![
        Box::new(RowBinaryEquilibrium::new()),
        Box::new(ColBinaryEquilibrium::new())
    ];
    let mut solver = CSPSolver::new(binary, binary_constraints);
    let binary = match solver.solve() {
       Ok(binary) => {println!("Solved!\n{}", binary.str_repr()); binary},
        Err(_) => {println!("This problem is unsolvable"); exit(0)},
    };
}
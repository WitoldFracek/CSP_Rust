use std::ops::Range;
use std::path::Component::ParentDir;
use std::process::exit;
use crate::constraints::Constraint;
use crate::constraints::grid::{ColBinaryEquilibrium, ColUnique, RowBinaryEquilibrium, RowUnique, SudokuSquare};
use crate::domains::{Domain, FixedDomain, RangeDomain};
use crate::presentation::sudoku::sudoku_repr;
use crate::problem::CSP;
use crate::problem::grid::Grid;
use crate::solver::CSPSolver;

mod constraints;
mod domains;
mod problem;
mod solver;
mod presentation;


fn sudoku_domains_from_vec(values: Vec<Vec<u8>>) -> Vec<Vec<Box<dyn Domain<Item=u8>>>> {
    let len = values.len() as u8;
    let mut ret = Vec::with_capacity(values.len() * values.len());
    for row in values {
        let mut domain_row = Vec::<Box<dyn Domain<Item=u8>>>::new();
        for elem in row {
            if elem != 0 {
              domain_row.push(Box::new(FixedDomain::new(elem))) ;
            } else {
                domain_row.push(Box::new(RangeDomain::new(1, len)));
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
    let _empty = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];
    let sudoku_init = vec![
        vec![0, 1, 0, 0, 9, 5, 0, 0, 8],
        vec![9, 0, 0, 0, 0, 0, 0, 0, 4],
        vec![3, 0, 7, 0, 1, 0, 9, 0, 0],
        vec![0, 0, 0, 8, 0, 3, 2, 0, 0],
        vec![0, 2, 0, 5, 6, 0, 0, 0, 0],
        vec![0, 0, 3, 0, 7, 2, 0, 5, 0],
        vec![2, 0, 0, 0, 5, 6, 8, 0, 0],
        vec![0, 7, 0, 0, 0, 4, 0, 1, 6],
        vec![8, 0, 0, 0, 0, 0, 0, 0, 2],
    ];

    let sudoku_init = vec![
        vec![0, 1, 0, 0, 9, 5, 0, 0, 8],
        vec![9, 0, 0, 0, 0, 0, 0, 0, 4],
        vec![3, 0, 7, 0, 1, 0, 9, 0, 0],
        vec![0, 0, 0, 8, 0, 3, 2, 0, 0],
        vec![0, 2, 0, 5, 6, 0, 0, 0, 0],
        vec![0, 0, 3, 0, 7, 2, 0, 5, 0],
        vec![2, 0, 0, 0, 5, 6, 8, 0, 0],
        vec![0, 7, 0, 0, 0, 4, 0, 0, 6],
        vec![8, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let binary_init = vec![
        vec![-1, -1, -1, 1, -1, -1],
        vec![0, -1, -1, 0, -1, -1],
        vec![-1, -1, -1, 0, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![0, -1, -1, -1, -1, -1],
        vec![0, 1, -1, -1, -1, -1],
    ];


    let mut sudoku = Grid::from_domains(sudoku_domains_from_vec(sudoku_init));
    let mut binary = Grid::from_domains(binary_domains_from_vec(binary_init));
    let mut azul = Grid::from_domains(sudoku_domains_from_vec(_empty));

    println!("Azul");
    let azul_constraints: Vec<Box<dyn Constraint<Grid<u8>>>> = vec![
        Box::new(RowUnique::new()),
        Box::new(ColUnique::new())
    ];

    let mut solver = CSPSolver::new(&mut azul, azul_constraints);
    // let count = solver.how_many_solutions();
    // println!("Azul has {count} solutions");

    println!("Sudoku:\n{}", sudoku_repr(&sudoku));
    let sudoku_constraints: Vec<Box<dyn Constraint<Grid<u8>>>> = vec![
        Box::new(RowUnique::new()),
        Box::new(ColUnique::new()),
        Box::new(SudokuSquare::new())
    ];

    let mut solver = CSPSolver::new(&mut sudoku, sudoku_constraints);
    // let count = solver.how_many_solutions();
    // println!("{count}");
    let _sudoku = match solver.solve() {
        Ok(sudoku) => {println!("Solved!\n{}", sudoku_repr(&sudoku)); sudoku},
        Err(_) => {println!("This problem is unsolvable"); exit(0)},
    };

    println!("Binary:\n{}", binary.to_string());
    let binary_constraints: Vec<Box<dyn Constraint<Grid<i8>>>> = vec![
        Box::new(RowBinaryEquilibrium::new()),
        Box::new(ColBinaryEquilibrium::new())
    ];
    let mut solver = CSPSolver::new(&mut binary, binary_constraints);
    let _binary = match solver.solve() {
       Ok(binary) => {println!("Solved!\n{}", binary.to_string()); binary},
        Err(_) => {println!("This problem is unsolvable"); exit(0)},
    };
}
use std::fmt::{Debug, Display};
use std::slice::Iter;
use crate::domains::{Domain};
use crate::{CSP, RangeDomain};
use crate::constraints::Constraint;

#[derive(Debug)]
pub struct Sudoku {
    domains: Vec<Vec<RangeDomain<u8>>>,
    values: Vec<Vec<u8>>,
    row_i: usize,
    col_i: usize,
}

impl CSP for Sudoku {
    fn forward(&mut self) -> Result<bool, String> {
        if self.is_finished() {
            return Ok(true);
        }
        if self.values[self.row_i][self.col_i] == 0 {
            self.domains[self.row_i][self.col_i].next();
            self.values[self.row_i][self.col_i] = self.domains[self.row_i][self.col_i].value();
            return Ok(false);
        }
        self.next_cell()
    }

    fn backward(&mut self) -> Result<(), String> {
        let mut updated = false;
        while !updated {
            if self.domains[self.row_i][self.col_i].is_fixed() {

            }
            if self.domains[self.row_i][self.col_i].has_next() {
                self.domains[self.row_i][self.col_i].next();
                self.values[self.row_i][self.col_i] = self.domains[self.row_i][self.col_i].value();
                updated = true;
            } else {
                self.domains[self.row_i][self.col_i].reset();
                self.values[self.row_i][self.col_i] = 0;
                if let Err(message) = self.prev_cell() {
                    return Err(message);
                }
            }
        }
        Ok(())
    }
}

impl Sudoku {
    fn are_vec_values_in_range(values: &Vec<Vec<u8>>) -> bool {
        for row in values {
            for &elem in row {
                match elem {
                    0..=9 => {},
                    _ => return false,
                }
            }
        }
        true
    }

    pub fn str_repr(&self) -> String {
        let mut ret = String::with_capacity(252);
        for row in &self.values {
            for &elem in row {
                if elem != 0 {
                    ret = format!("{ret} {} ", elem);
                } else {
                    ret = format!("{ret} _ ");
                }
            }
            ret.push('\n');
        }
        ret
    }

    fn next_cell(&mut self) -> Result<bool, String> {
        if self.row_i == 9 {
            return Ok(true);
        }
        self.col_i += 1;
        if self.col_i == 9 {
            self.col_i = 0;
            self.row_i += 1;
        }
        Ok(false)
    }

    fn prev_cell(&mut self) -> Result<(), String> {
        if self.row_i == 0 && self.col_i == 0 {
            return Err(String::from("Cannot go backward from cell (0, 0)"));
        }
        if self.col_i == 0 {
            self.col_i = 9;
            self.row_i -= 1;
        }
        self.col_i -= 1;
        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.row_i == 9
    }
}

impl From<Vec<Vec<u8>>> for Sudoku {
    fn from(value: Vec<Vec<u8>>) -> Self {
        if !Self::are_vec_values_in_range(&value) {
            panic!("Not all values are in acceptable range. Values must be between 0 and 9.");
        }
        let mut domains = Vec::with_capacity(9);
        let mut values = Vec::with_capacity(9);
        for row in value {
            let mut domains_row = Vec::with_capacity(9);
            let mut values_row = Vec::with_capacity(9);
            for arg in row {
                if arg == 0 {
                    domains_row.push(RangeDomain::new(0, 9));
                } else {
                    domains_row.push(RangeDomain::new(arg, arg));
                }
                values_row.push(arg);
            }
            domains.push(domains_row);
            values.push(values_row);
        }
        Self { domains, values, row_i: 0, col_i: 0 }
    }
}

trait SudokuIterUnique {
    fn unique<'b, It>(row_iter: It) -> bool where It: Iterator<Item=&'b u8> {
        let mut nums = [0_u8; 10];
        row_iter.for_each(|&x| nums[x as usize] += 1);
        nums.iter().skip(1).all(|&x| x < 2)
    }
}

pub struct SudokuRowConstraint<'a> {
    sudoku: &'a Sudoku,
}

impl SudokuIterUnique for SudokuRowConstraint<'_> { }

impl <'a> SudokuRowConstraint<'a> {
    pub fn new(sudoku: &'a Sudoku) -> Self {
        Self { sudoku }
    }

    fn are_rows_correct(&self) -> bool {
       self.sudoku.values.iter().all(|row| Self::unique(row.iter()))
    }
}

impl SudokuIterUnique for SudokuColConstraint<'_> { }

impl Constraint for SudokuRowConstraint<'_> {
    fn is_satisfied(&self) -> bool {
        self.are_rows_correct()
    }
}


pub struct SudokuColConstraint<'a> {
    sudoku: &'a Sudoku
}

impl <'a> SudokuColConstraint<'a> {
    pub fn new(sudoku: &'a Sudoku) -> Self {
        Self { sudoku }
    }

    fn are_cols_correct(&self) -> bool {
        (0..9).all(|i| Self::unique(self.sudoku.values.iter().map(|row| &row[i])))
    }
}

impl Constraint for SudokuColConstraint<'_> {
    fn is_satisfied(&self) -> bool {
        self.are_cols_correct()
    }
}

pub struct SudokuSquareConstraint<'a> {
    sudoku: &'a Sudoku
}

impl SudokuIterUnique for SudokuSquareConstraint<'_> { }

impl <'a> SudokuSquareConstraint<'a> {
    pub fn new(sudoku: &'a Sudoku) -> Self {
        Self { sudoku }
    }

    fn are_squares_correct(&self) -> bool {
        let square_it = |i, j| self.sudoku.values[i..i+3].iter().flat_map(move |row| &row[j..j+3]);
        (0..3).all(|i| (0..3).all(|j| Self::unique(square_it(i * 3, j * 3))))
    }
}

impl Constraint for SudokuSquareConstraint<'_> {
    fn is_satisfied(&self) -> bool {
        self.are_squares_correct()
    }
}

pub fn is_sudoku_satisfied(problem: &Sudoku) -> bool {
    let row_constraint = SudokuRowConstraint::new(problem);
    let col_constraint = SudokuColConstraint::new(problem);
    let square_constraint = SudokuSquareConstraint::new(problem);
    if !row_constraint.is_satisfied() {
        return false;
    }
    if !col_constraint.is_satisfied() {
        return false;
    }
    if !square_constraint.is_satisfied() {
        return false;
    }
    true
}
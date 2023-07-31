use std::fmt::{Debug, Display, Formatter};
use crate::domains::Domain;
use crate::CSP;

pub struct Grid<T> {
    pub domains: Vec<Vec<Box<dyn Domain<Item=T>>>>,
    row_i: usize,
    col_i: usize,
    pub size: usize,
}

impl <T: PartialEq + Copy> CSP for Grid<T>{
    fn forward(&mut self) -> Result<bool, String> {
        if self.is_finished() {
            return Ok(true);
        }
        if let None = self.domains[self.row_i][self.col_i].value() {
            self.domains[self.row_i][self.col_i].next();
            return Ok(false);
        }
        self.next_cell()
    }

    fn backward(&mut self) -> Result<(), String> {
        let mut updated = false;
        while !updated {
            if self.domains[self.row_i][self.col_i].has_next() {
                self.domains[self.row_i][self.col_i].next();
                updated = true;
            } else {
                self.domains[self.row_i][self.col_i].reset();
                if let Err(message) = self.prev_cell() {
                    return Err(message);
                }
            }
        }
        Ok(())
    }
}

impl <T: PartialEq + Copy> Grid<T> {
    pub fn from_domains(domains: Vec<Vec<Box<dyn Domain<Item=T>>>>) -> Self {
        let size = domains.len();
        Self {domains, size, col_i: 0, row_i: 0}
    }

    fn next_cell(&mut self) -> Result<bool, String> {
        if self.row_i == self.size {
            return Ok(true)
        }
        self.col_i += 1;
        if self.col_i == self.size {
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
            self.col_i = self.size;
            self.row_i -= 1;
        }
        self.col_i -= 1;
        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.row_i == self.size
    }
}

impl <T: PartialEq + Display> Grid<T> {
    pub fn str_repr(&self) -> String {
        let mut ret = String::new();
        for row in &self.domains {
            for domain in row {
                match domain.value() {
                    None => ret = format!("{ret} _ "),
                    Some(value) => ret = format!("{ret} {} ", value),
                }
            }
            ret.push('\n');
        }
        ret
    }
}

impl <T> Debug for Grid<T> where T: PartialEq + Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str_repr())
    }
}

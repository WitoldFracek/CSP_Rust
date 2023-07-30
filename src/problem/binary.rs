// use num_traits::real::Real;
// use crate::domains::Domain;
// use crate::{Constraint, CSP, RangeDomain};
//
// pub struct Binary {
//     domains: Vec<Vec<RangeDomain<i8>>>,
//     row_i: usize,
//     col_i: usize,
//     size: usize,
// }
//
// impl CSP for Binary {
//     fn forward(&mut self) -> Result<bool, String> {
//         if self.is_finished() {
//             return Ok(true);
//         }
//         if self.domains[self.row_i][self.col_i].value() == -1 {
//             self.domains[self.row_i][self.col_i].next();
//             return Ok(false);
//         }
//         self.next_cell()
//     }
//
//     fn backward(&mut self) -> Result<(), String> {
//         let mut updated = false;
//         while !updated {
//             if self.domains[self.row_i][self.col_i].has_next() {
//                 self.domains[self.row_i][self.col_i].next();
//                 updated = true;
//             } else {
//                 self.domains[self.row_i][self.col_i].reset();
//                 if let Err(message) = self.prev_cell() {
//                     return Err(message);
//                 }
//             }
//         }
//         Ok(())
//     }
// }
//
// impl Binary {
//     fn are_values_in_range(values: &Vec<Vec<i8>>) -> bool {
//         let size = values.len();
//         if !values.iter().all(|row| row.len() == size) { return false; }
//         values.iter().flat_map(|row| row).all(|&x| x == 0 || x == 1 || x == -1)
//     }
//
//     fn get_domain_vec(values: &Vec<Vec<i8>>) -> Vec<Vec<RangeDomain<i8>>> {
//         values
//             .iter()
//             .map(|row| row
//                 .iter()
//                 .map(|&x| if x == -1 { RangeDomain::new(-1, 1) } else { RangeDomain::new(x, x) })
//                 .collect::<Vec<RangeDomain<i8>>>())
//             .collect()
//     }
//
//     pub fn str_repr(&self) -> String {
//         let mut ret = String::new();
//         for row in &self.domains {
//             for domain in row {
//                 if domain.value() == -1 {
//                     ret = format!("{ret} _ ");
//                 } else {
//                     ret = format!("{ret} {} ", domain.value())
//                 }
//             }
//             ret.push('\n');
//         }
//         ret
//     }
//
//     fn next_cell(&mut self) -> Result<bool, String> {
//         if self.row_i == self.size {
//             return Ok(true)
//         }
//         self.col_i += 1;
//         if self.col_i == self.size {
//             self.col_i = 0;
//             self.row_i += 1;
//         }
//         Ok(false)
//     }
//
//     fn prev_cell(&mut self) -> Result<(), String> {
//         if self.row_i == 0 && self.col_i == 0 {
//             return Err(String::from("Cannot go backward from cell (0, 0)"));
//         }
//         if self.col_i == 0 {
//             self.col_i = self.size;
//             self.row_i -= 1;
//         }
//         self.col_i -= 1;
//         Ok(())
//     }
//
//     fn is_finished(&self) -> bool {
//         self.row_i == self.size
//     }
// }
//
// impl From<Vec<Vec<i8>>> for Binary {
//     fn from(value: Vec<Vec<i8>>) -> Self {
//         if !Binary::are_values_in_range(&value) {
//             panic!("Not all values are in the acceptable range.\
//             Expects to have the same number of elements in each Vec and the number of Vecs \
//             should be equal to the number of elements in each Vec. \
//             Values must be between -1 and 1.")
//         }
//         let domains = Binary::get_domain_vec(&value);
//         let len = domains.len();
//         Self { domains, size: len, row_i: 0, col_i: 0}
//     }
// }
//
// trait BinaryIterEquilibrium {
//     fn in_equilibrium<I>(iter: I) -> bool where I: Iterator<Item=i8> {
//         let mut counter = [0_i8; 3];
//         let mut sum = 0;
//         iter.for_each(|x| {
//             sum += 1;
//             if x == -1 {counter[2] += 1} else {counter[x as usize] += 1 }
//         });
//         (counter[0] - counter[1]).abs() <= counter[2] + (sum % 2)
//     }
// }
//
// pub struct BinaryRowConstraint {
// }
//
// impl BinaryIterEquilibrium for BinaryRowConstraint { }
//
// impl BinaryRowConstraint {
//     pub fn new() -> Self {
//         Self { }
//     }
//
//     fn is_row_ok(&self, binary: &Binary) -> bool {
//         binary.domains.iter().all(|x| Self::in_equilibrium(x.iter().map(|d| d.value())))
//     }
// }
//
// impl Constraint<Binary> for BinaryRowConstraint {
//     fn is_satisfied(&self, problem: &Binary) -> bool {
//         self.is_row_ok(problem)
//     }
// }
//
// pub struct BinaryColConstraint {
//
// }
//
// impl BinaryIterEquilibrium for BinaryColConstraint { }
//
// impl BinaryColConstraint {
//     pub fn new() -> Self {
//         Self { }
//     }
//
//     fn is_col_ok(&self, binary: &Binary) -> bool {
//         (0..binary.size).all(|i| Self::in_equilibrium(binary.domains.iter().map(|row| row[i].value())))
//     }
// }
//
// impl Constraint<Binary> for BinaryColConstraint {
//     fn is_satisfied(&self, problem: &Binary) -> bool {
//         self.is_col_ok(problem)
//     }
// }
use std::fmt::Debug;
use crate::{Constraint, CSP};

pub struct CSPSolver<P: CSP> {
    problem: P,
    constraints: Vec<Box<dyn Constraint<P>>>
}

impl <P: CSP + Debug> CSPSolver<P> {

    pub fn new(problem: P, constraints: Vec<Box<dyn Constraint<P>>>) -> Self {
        Self { problem, constraints }
    }

    fn all_satisfied(&self) -> bool {
        self.constraints.iter().all(|c| c.is_satisfied(&self.problem))
    }

    pub fn solve(mut self) -> Result<P, String> {
        let mut solved = false;
        while !solved {
            if !self.all_satisfied() {
                // println!("{:?}", self.problem);
                match self.problem.backward() {
                    Ok(()) => {},
                    Err(message) => return Err(format!("Problem cannot be solved. {message}")),
                }
            } else {
                match self.problem.forward() {
                    Ok(arg) => {solved = arg},
                    Err(message) => return Err(format!("Problem cannot be solved. {message}"))
                }
            }
        }
        Ok(self.problem)
    }
}
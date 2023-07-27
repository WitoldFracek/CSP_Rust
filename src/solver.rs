use crate::{Constraint, CSP};

pub struct CSPSolver<P: CSP> {
    problem: P,
    // is_satisfied: fn(&P) -> bool,
    constraints: Vec<Box<dyn Constraint<P>>>
}

impl <P: CSP> CSPSolver<P> {

    // pub fn new(problem: P, constraints_factory: fn(&P) -> Vec<Box<dyn Constraint>>) -> Self {
    //     let mut ret = Self { problem, constraints: vec![] };
    //     ret.constraints = (constraints_factory)(&ret.problem);
    //     ret
    // }

    pub fn new(problem: P, constraints: Vec<Box<dyn Constraint<P>>>) -> Self {
        Self { problem, constraints }
    }

    fn all_satisfied(&self) -> bool {
        for cons in &self.constraints {
            if !cons.is_satisfied(&self.problem) { return false; }
        }
        true
    }

    // pub fn new(problem: P, is_satisfied: fn(&P) -> bool) -> Self {
    //     Self { problem, is_satisfied }
    // }

    pub fn solve(mut self) -> Result<P, String> {
        let mut solved = false;
        while !solved {
            if !self.all_satisfied() {
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
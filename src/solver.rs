use crate::{Constraint, CSP};

pub struct CSPSolver<P: CSP> {
    problem: P,
    // is_satisfied: fn(&P) -> bool,
    constraints: Vec<Box<dyn Constraint>>
}

impl <P: CSP> CSPSolver<P> {

    pub fn new(problem: P, constraints_factory: fn(&P) -> Vec<Box<dyn Constraint>>) -> Self {
        let constraints = (constraints_factory)(&problem);
        Self { problem, constraints }
    }

    fn all_satisfied(&self) -> bool {
        self.constraints.iter().all(|c| c.is_satisfied())
    }

    // pub fn new(problem: P, is_satisfied: fn(&P) -> bool) -> Self {
    //     Self { problem, is_satisfied }
    // }

    pub fn solve(&mut self) -> Result<(), String> {
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
        Ok(())
    }
}
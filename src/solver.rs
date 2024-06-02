use std::fmt::Debug;
use crate::{Constraint, CSP};

pub struct CSPSolver<'a, P: CSP> {
    problem: &'a mut P,
    constraints: Vec<Box<dyn Constraint<P>>>
}

impl <'a, P: CSP + Debug> CSPSolver<'a, P> {

    pub fn new(problem: &'a mut P, constraints: Vec<Box<dyn Constraint<P>>>) -> Self {
        Self { problem, constraints }
    }

    fn all_satisfied(&self) -> bool {
        self.constraints.iter().all(|c| c.is_satisfied(&self.problem))
    }

    pub fn solve(&mut self) -> Result<&'_ P, String> {
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

    pub fn how_many_solutions(&mut self) -> i32 {
        let mut ret = 0;
        while let Ok(solution) = self.solve() {
            ret += 1;
            match self.problem.backward() {
                Ok(_) => {},
                Err(_) => break,
            }
        }
        ret
    }
}

impl <'a, P: CSP + Debug + Clone> CSPSolver<'a, P> {
    pub fn solve_all(&mut self) -> Vec<P> {
        let mut ret = Vec::new();
        while let Ok(solution) = self.solve() {
            ret.push(self.problem.clone())
        }
        ret
    }
}
pub mod grid;

pub trait CSP {
    fn forward(&mut self) -> Result<bool, String>;
    fn backward(&mut self) -> Result<(), String>;
}
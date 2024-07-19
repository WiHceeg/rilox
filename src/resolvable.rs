use crate::token::Token;

pub trait Resolvable {
    fn name(&self) -> &Token;
    fn set_distance(&mut self, distance: usize);
    fn get_distance(&self) -> Option<usize>;
}
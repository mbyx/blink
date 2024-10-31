
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Request {
    Yield(usize),
}
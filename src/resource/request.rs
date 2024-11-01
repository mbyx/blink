/// Represents an I/O Request made by a task.
///
/// Tasks that make a request will be blocked until that request
/// has been completed.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Request {
    Yield(usize),
}

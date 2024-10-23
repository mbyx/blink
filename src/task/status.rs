#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum TaskStatus {
    New,
    Ready,
    Running,
    Blocked,
    Exited,
    Suspended,
}

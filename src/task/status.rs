/// Represents one of six states that a task can be in. These are adapted from
/// a process control structure as specified in
/// 'William Stalling's Operating Systems, Internals and Design Principles.'
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum TaskStatus {
    /// A task has just been created but is not ready to be ran.
    New,
    /// A task is ready to be ran and waits for an open slot.
    Ready,
    /// A task is currently running on the processor cores.
    Running,
    /// A task is waiting for an IO operation to complete.
    Blocked,
    /// A task has completed or it has irrevocably failed to complete.
    Exited,
    /// The task has revocably failed to complete and is stored but not saved.
    Suspended,
}

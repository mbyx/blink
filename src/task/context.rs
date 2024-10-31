use getset::{Getters, MutGetters, Setters};
use std::time;

use super::{TaskPriority, TaskStatus};
use crate::resource::Request;

/// Represents the additional information or context required for scheduling.
///
/// This structure has everything needed to properly schedule tasks to avoid
/// conflicts and maximise usage of processor cores and time.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[derive(Getters, Setters, MutGetters)]
pub struct TaskContext {
    /// The friendly display name of the task.
    #[getset(get = "pub")]
    name: String,
    /// The unique id assigned to each task.
    #[getset(get = "pub")]
    id: uuid::Uuid,
    /// The execution priority of the task.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    priority: TaskPriority,
    /// The current state of the task.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    state: TaskStatus,
    /// The current step that the task is on.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    program_counter: usize,
    /// A list of which pins as resources are assigned to this task.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pins_used: Vec<i32>,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    block_requests: Vec<Request>,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    last_run_timestamp: time::Instant,

    // TODO: Add support for storing:
    // TODO: - Context Data (Data present in registers, used for saving state when restarting a process from middle)
    // TODO: - IO Status Info (List of IO requests, devices assigned to it, list of files used, etc.)
    // TODO: - Accounting Info (Processor time used so far, clock time used, time limits before its aborted, etc.)
}

impl TaskContext {
    /// Create a new context with the name of the task and its priority.
    ///
    /// All tasks created this way are set the the 'New' state. By default
    /// no resources are assigned to the task.
    pub fn new(name: String, priority: TaskPriority) -> Self {
        Self {
            name,
            id: uuid::Uuid::new_v4(),
            priority,
            state: TaskStatus::New,
            program_counter: 0,
            pins_used: vec![],
            block_requests: vec![],
            last_run_timestamp: time::Instant::now(),
        }
    }
}

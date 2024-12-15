use esp_idf_hal::gpio::Level;
use getset::{Getters, MutGetters, Setters};
use std::time;

use super::{TaskPriority, TaskStatus};
use crate::resource::Request;

/// Represents the additional information or context required for scheduling.
///
/// This structure has everything needed to properly schedule tasks to avoid
/// conflicts and maximise usage of processor cores and time.
#[derive(Debug, PartialEq, Eq, Clone, Getters, Setters, MutGetters)]
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

    /// A list of I/O requests to be processed before the task can run.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    block_requests: Vec<Request>,

    /// The timestamp at which this task was last stepped.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    last_run_timestamp: time::Instant,

    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    last_pin_read_level_register: Level
    // TODO: Add support for storing:
    // - Context Data (Data present in registers, used for saving state when restarting a process from middle)
    // - IO Status Info (List of IO requests, devices assigned to it, list of files used, etc.)
    // - Accounting Info (Processor time used so far, clock time used, time limits before its aborted, etc.)
    // TODO: Consider how to manage resources carefully so that there is not dead time.
}

impl Default for TaskContext {
    fn default() -> Self {
        Self {
            name: "".into(),
            id: uuid::Uuid::new_v4(),
            priority: TaskPriority::Low,
            state: TaskStatus::New,
            program_counter: 0,
            pins_used: vec![],
            block_requests: vec![],
            last_pin_read_level_register: Level::Low,
            last_run_timestamp: time::Instant::now(),
        }
    }
}

impl TaskContext {
    /// Create a new context with the name of the task and its priority.
    ///
    /// All tasks created this way are set the the 'New' state. By default
    /// no resources are assigned to the task.
    pub fn new(name: String, priority: TaskPriority) -> Self {
        Self {
            name,
            priority,
            ..Default::default()
        }
    }
}

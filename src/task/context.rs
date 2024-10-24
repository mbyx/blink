use super::{TaskPriority, TaskStatus};

/// Represents the additional information or context required for scheduling.
///
/// This structure has everything needed to properly schedule tasks to avoid
/// conflicts and maximise usage of processor cores and time.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TaskContext {
    /// The friendly display name of the task.
    name: String,
    /// The unique id assigned to each task.
    id: uuid::Uuid,
    /// The execution priority of the task.
    priority: TaskPriority,
    /// The current state of the task.
    state: TaskStatus,
    /// The current step that the task is on.
    program_counter: usize,
    /// A list of which pins as resources are assigned to this task.
    pins_used: Vec<i32>,
    // TODO: Add support for storing:
    // TODO: - Memory Pointers (To the program code and data, probably not needed)
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
        }
    }

    /// Get a mutable reference to the pins the task has been assigned.
    pub fn pins(&mut self) -> &mut Vec<i32> {
        &mut self.pins_used
    }

    /// Get the friendly display name of the task.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the unique id of the task.
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    /// Get the execution priority of the task.
    pub fn priority(&self) -> TaskPriority {
        self.priority
    }

    /// Get the current state of the task.
    pub fn state(&self) -> TaskStatus {
        self.state
    }

    /// Get the current step the task is on.
    pub fn program_counter(&self) -> usize {
        self.program_counter
    }
}

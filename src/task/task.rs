use super::{TaskContext, TaskPriority, TaskStep};
use crate::resource::{ResourceManager, TaskResource};

/// Represents a task in the scheduler, which is the simplest unit of work.
///
/// A task consists of a number of steps that it performs when run, as well as
/// some context to it, such as which pins or other IO does it hold access to,
/// a unique id representing it, it's state and priority, etc.
///
/// When a task is created, no resources are assigned to it. Instead, resources
/// must be manually assigned or acquired by the task on creation. If this is
/// not done, access to the resource will be denied and the task will immediately
/// enter an aborted or exited state.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Task {
    /// Additional accounting information required for proper task management.
    context: TaskContext,
    /// A list of the smallest instructions that a task can perform.
    // TODO: Allow the use of async functions directly by abusing 'Future'.
    steps: Vec<TaskStep>,
    // TODO: Add an 'interval' to the task as well as 'shots' to determine
    // TODO: when to run a step in the task as well as how many times
    // TODO: the task runs completely.
}

impl Task {
    /// Create a new task with the given name, priority, and instructions to perform.
    ///
    /// This task does not have any resources assigned to it and these must be assigned
    /// manually. Additionally, the task starts out by default in the 'New' state.
    pub fn new(name: &str, priority: TaskPriority, steps: Vec<TaskStep>) -> Self {
        Self {
            context: TaskContext::new(name.into(), priority),
            steps,
        }
    }

    /// Run a task to completion without yielding for any reason, returning the id of the task.
    ///
    /// This method is not reccomended to be used directly as it bypasses the task
    /// scheduler entirely, causing no time gains to be had.
    ///
    /// If a task had some steps performed by the scheduler before being manually
    /// ran, then the task will continue from that point on instead of restarting.
    pub fn run(&mut self, manager: &mut ResourceManager) -> anyhow::Result<uuid::Uuid> {
        for step in &mut self.steps[self.context.program_counter()..] {
            step.execute(&mut self.context, manager)?;
        }
        Ok(self.context.id())
    }

    /// Assign or acquire a resource to be usable by this task.
    ///
    /// This method saves additional context to the task regarding which resource
    /// it has access to. A resource can be anything, from pins to files.
    ///
    /// All this method does is perform book keeping, actual management of resources
    /// will always be done by the ResourceManager, and in no circumstance should
    /// actual resources be used directly.
    ///
    /// Returns the task itself for convenience.
    pub fn acquire_resource(mut self, resource: TaskResource) -> Self {
        match resource {
            TaskResource::Pin(pin) => (*self.context.pins()).push(pin),
        }

        self
    }
}

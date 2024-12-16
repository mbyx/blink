use super::{Shot, TaskContext, TaskPriority, TaskStep};
use crate::resource::{ResourceManager, TaskResource};

use anyhow::Context;
use getset::{Getters, MutGetters, Setters};
use typed_builder::TypedBuilder;

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
#[derive(Debug, PartialEq, Eq, Clone, Getters, Setters, MutGetters, TypedBuilder)]
pub struct Task {
    /// Additional accounting information required for proper task management.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    context: TaskContext,
    /// A list of the smallest instructions that a task can perform.
    #[getset(get = "pub")]
    steps: Vec<TaskStep>,
    /// The number of times the task can be fully run.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    shots: Shot,
    // TODO: Move shots into context, or structure tasks to be more in line with the book.
    // i.e., the three parts of a process.
    // TODO: Allow the use of async functions directly by abusing 'Future'.
    // TODO: Add an 'interval' to the task to determine when to run a step in the task.
}

impl<'a> Task {
    /// Create a new task with the given name, priority, and instructions to perform.
    ///
    /// This task does not have any resources assigned to it and these must be assigned
    /// manually. Additionally, the task starts out by default in the 'New' state.
    pub fn new(name: &str, priority: TaskPriority, shots: Shot, steps: Vec<TaskStep>) -> Self {
        Self {
            context: TaskContext::new(name.into(), priority),
            steps,
            shots,
        }
    }

    /// Run a task to completion without yielding for any reason.
    ///
    /// This method is not recommended to be used directly as it bypasses the task
    /// scheduler entirely, causing no time gains to be had.
    ///
    /// If a task had some steps performed by the scheduler before being manually
    /// ran, then the task will continue from that point on instead of restarting.
    pub fn run(&mut self, manager: &mut ResourceManager<'a>) -> anyhow::Result<()> {
        if *self.context.program_counter() >= self.steps.len() {
            *self.context.program_counter_mut() = 0;
            self.shots -= 1;
        }

        let steps = &mut self.steps[*self.context.program_counter()..];
        for step in steps {
            step.execute(&mut self.context, manager)?;
            *self.context.program_counter_mut() += 1;
        }

        self.shots -= 1;
        Ok(())
    }

    /// Run a single step in a task, then yield to the scheduler.
    ///
    /// This method is the primary method used by the scheduler to run tasks, providing
    /// gains in terms of time if there are multiple tasks that continuously yield.
    ///
    /// This method also saves the current step that it is on so that it will always
    /// start where it left off.
    pub fn step(&mut self, manager: &mut ResourceManager) -> anyhow::Result<()> {
        if *self.context.program_counter() >= self.steps.len() {
            *self.context.program_counter_mut() = 0;
            self.shots -= 1;
        }

        let task_step = self
            .steps
            .get_mut(*self.context.program_counter())
            .context("Program counter set outside bounds.")?;

        *self.context.program_counter_mut() += 1;

        let result = task_step.execute(&mut self.context, manager)?;
        if let Some(request) = result {
            self.context_mut().block_requests_mut().push(request);
        }
        Ok(())
    }

    /// Assign a resource to be usable by this task.
    ///
    /// This method saves additional context to the task regarding which resource
    /// it has access to. A resource can be anything, from pins to files.
    ///
    /// All this method does is perform book keeping, actual management of resources
    /// will always be done by the ResourceManager, and in no circumstance should
    /// actual resources be used directly.
    ///
    /// Returns the task itself for convenience.
    pub fn assign(mut self, resource: TaskResource) -> Self {
        match resource {
            TaskResource::Pin(pin) => self.context.pins_used_mut().push(pin),
        }

        self
    }
}

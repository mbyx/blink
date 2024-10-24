use super::{context::TaskContext, priority::TaskPriority, step::TaskStep};
use crate::resource::{manager::ResourceManager, resource::TaskResource};

pub struct Task {
    context: TaskContext,
    steps: Vec<TaskStep>,
}

impl Task {
    pub fn new(name: &str, priority: TaskPriority, steps: Vec<TaskStep>) -> Self {
        Self {
            context: TaskContext::new(name.into(), priority),
            steps,
        }
    }

    pub fn run(&mut self, manager: &mut ResourceManager) -> anyhow::Result<()> {
        for step in &mut self.steps[self.context.program_counter()..] {
            step.execute(&mut self.context, manager)?;
        }
        Ok(())
    }

    pub fn acquire_resource(mut self, resource: TaskResource) -> Self {
        match resource {
            TaskResource::Pin(pin) => (*self.context.pins()).push(pin),
        }
        self
    }
}

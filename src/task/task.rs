use esp_idf_hal::peripheral::Peripheral;

use super::{context::TaskContext, priority::TaskPriority, resource::TaskResource, step::TaskStep};

pub struct Task<'a> {
    context: TaskContext<'a>,
    steps: Vec<TaskStep>,
}

impl<'a> Task<'a> {
    pub fn new(name: &str, priority: TaskPriority, steps: Vec<TaskStep>) -> Self {
        Self {
            context: TaskContext::new(name.into(), priority),
            steps,
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        for step in &mut self.steps[self.context.program_counter()..] {
            step.execute(&mut self.context)?;
        }
        Ok(())
    }

    pub fn acquire_resource(mut self, resource: TaskResource<'a>) -> Self {
        match resource {
            TaskResource::Pin(pin) => (*self.context.pins()).push(pin.into_ref()),
        }
        self
    }
}

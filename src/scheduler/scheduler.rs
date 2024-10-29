use crate::{resource::ResourceManager, task::Task};

pub struct TaskScheduler {
    manager: ResourceManager<'static>,
    // TODO: Sort the tasks using multiple queues for different priorities and states.
    tasks: Vec<Task>,
}

impl TaskScheduler {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            manager: ResourceManager::new()?,
            tasks: vec![],
        })
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            // TODO: Implement a better scheduling algorithm.
            // What this does is essentially sort by priority, and then run each task to completion.
            // What it should do at worst is sort by priority, then sort by ready tasks.
            // Then it should step through the first task and check if it wants to block. In which
            // case it will switch to the other tasks, always checking if the highest priority task
            // is running or blocked and when it will be unblocked.
            self.tasks.sort_by(|a, b| a.priority().cmp(&b.priority()));
            for task in &mut self.tasks {
                task.run(&mut self.manager)?;
            }
        }
    }
}

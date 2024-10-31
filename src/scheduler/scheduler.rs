use std::{collections::HashMap, time};

use anyhow::Context;

use crate::{resource::{Request, ResourceManager}, task::{Shot, Task, TaskPriority, TaskStatus}};

pub struct TaskScheduler {
    manager: ResourceManager<'static>,
    // TODO: Sort by priorities as well. HashMap<TaskStatus, HashMap<TaskPriority, Vec<Task>>
    // tasks[TaskStatus::Ready][TaskPriority::High]
    tasks: HashMap<TaskStatus, Vec<Task>>,
    queue_size: usize,
}

impl TaskScheduler {
    pub fn new() -> anyhow::Result<Self> {
        let mut tasks = HashMap::new();
        tasks.entry(TaskStatus::New).or_default();
        tasks.entry(TaskStatus::Ready).or_default();
        tasks.entry(TaskStatus::Running).or_default();
        tasks.entry(TaskStatus::Blocked).or_default();
        tasks.entry(TaskStatus::Exited).or_default();
        tasks.entry(TaskStatus::Suspended).or_default();

        
        Ok(Self {
            manager: ResourceManager::new()?,
            tasks,
            queue_size: 10
        })
    }

    pub fn add(&mut self, mut task: Task) {
        self.tasks.entry(*task.context().state())
            .or_default()
            .push(task);
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            // TODO: Implement a better scheduling algorithm.
            // What this does is essentially sort by priority, and then run each task to completion.
            // What it should do at worst is sort by priority, then sort by ready tasks.
            // Then it should step through the first task and check if it wants to block. In which
            // case it will switch to the other tasks, always checking if the highest priority task
            // is running or blocked and when it will be unblocked.
            let now = time::Instant::now();
            
            let [ready_tasks, new_tasks, blocked_tasks] = self.tasks.get_many_mut([
                &TaskStatus::Ready, &TaskStatus::New, &TaskStatus::Blocked
            ]).unwrap();
            
            // Resolve resource requests.
            // look at all blocked_tasks, check their requests, do some condition calling?
            // maybe need to add time stamps
            for blocked_task in blocked_tasks.iter_mut() {
                let elapsed = blocked_task.context_mut().last_run_timestamp().elapsed();
                blocked_task.context_mut().block_requests_mut().extract_if(|request| match request {
                        Request::Yield(ms) => {
                            elapsed >= time::Duration::from_millis(*ms as u64)
                        },
                    }).max(); // Just to consume it
            }

            // If you can add newly created tasks to the ready queue, do so.
            if ready_tasks.len() < self.queue_size {
                new_tasks.sort_by(|a, b| a.context().priority().cmp(&b.context().priority()));
                while (ready_tasks.len() < self.queue_size) && (!new_tasks.is_empty()) {
                    ready_tasks.push(new_tasks.pop().unwrap());
                }
            }

            // High priority at the end, ready to pop like a stack.
            ready_tasks.sort_by(|a, b| a.context().priority().cmp(&b.context().priority()));
            if !ready_tasks.is_empty() {
                let mut current_task = ready_tasks.pop().unwrap();
                *current_task.context_mut().state_mut() = TaskStatus::Running;
                
                current_task.step(&mut self.manager)?;
                if !current_task.context().block_requests().is_empty() {
                    *current_task.context_mut().state_mut() = TaskStatus::Blocked;
                    blocked_tasks.push(current_task);
                } else if *current_task.context().program_counter() >= current_task.steps().len() {
                    match current_task.shots() {
                        Shot::Custom(0) => *current_task.context_mut().state_mut() = TaskStatus::Exited,
                        Shot::Infinity | Shot::Custom(_) => ready_tasks.push(current_task),
                    }
                } else {
                    ready_tasks.push(current_task);
                }
            }
            
            // check how many blocked tasks have their request completed
            let mut free_tasks: Vec<_> = blocked_tasks
                .extract_if(|blocked_task| blocked_task.context().block_requests().is_empty()).collect();

            // If you can add blocked tasks that are free to the ready queue, do so.
            if ready_tasks.len() < self.queue_size {
                free_tasks.sort_by(|a, b| a.context().priority().cmp(&b.context().priority()));
                while (ready_tasks.len() < self.queue_size) && (!free_tasks.is_empty()) {
                    ready_tasks.push(free_tasks.pop().unwrap());
                }
            } else {
                blocked_tasks.extend(free_tasks);
            }

        }
    }
}

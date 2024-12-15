use std::{
    collections::HashMap,
    time::{self, Instant},
};


use crate::{
    resource::{Request, ResourceManager},
    task::{Shot, Task, TaskStatus},
};

pub struct TaskScheduler {
    manager: ResourceManager<'static>,
    // TODO: Make the scheduler use a state machine for task transitions.
    // TODO: Sort by priorities as well. HashMap<TaskStatus, HashMap<TaskPriority, Vec<Task>>
    // tasks[TaskStatus::Ready][TaskPriority::High]
    tasks: HashMap<TaskStatus, Vec<Task>>,
    queue_size: usize,
}

impl TaskScheduler {
    pub fn new() -> anyhow::Result<Self> {
        let mut tasks = HashMap::new();

        // TODO: Clean up.
        tasks.entry(TaskStatus::New).or_default();
        tasks.entry(TaskStatus::Ready).or_default();
        tasks.entry(TaskStatus::Running).or_default();
        tasks.entry(TaskStatus::Blocked).or_default();
        tasks.entry(TaskStatus::Exited).or_default();
        tasks.entry(TaskStatus::Suspended).or_default();

        Ok(Self {
            manager: ResourceManager::new()?,
            tasks,
            queue_size: 10,
        })
    }

    pub fn schedule(&mut self, task: Task) {
        self.tasks
            .entry(*task.context().state())
            .or_default()
            .push(task);
    }

    pub fn schedule_bulk(&mut self, tasks: Vec<Task>) {
        tasks.into_iter().for_each(|task| self.schedule(task));
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            // TODO: Implement a better scheduling algorithm.that uses preemption.
            // Preferable choose between either feedback or hrrt.

            // Currently, we are using a modified FCFS (First Come First Serve) approach
            // that uses priority queues.

            // What it does is essentially sort by priority, and run each task to completion.
            // In the case of an I/O block, preempt the task, when I/O succeeds, immediately swap.

            let [ready_tasks, new_tasks, blocked_tasks] = self
                .tasks
                .get_many_mut([&TaskStatus::Ready, &TaskStatus::New, &TaskStatus::Blocked])
                .unwrap();

            // Resolve I/O requests.
            // TODO: Move into resource manager or some kind of resource resolver.
            for blocked_task in blocked_tasks.iter_mut() {
                let elapsed = blocked_task.context_mut().last_run_timestamp().elapsed();
                blocked_task
                    .context_mut()
                    .block_requests_mut()
                    .extract_if(|request| match request {
                        Request::Yield(ms) => elapsed >= time::Duration::from_millis(*ms as u64),
                    })
                    .max(); // Just to consume it
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

                current_task
                    .context_mut()
                    .set_last_run_timestamp(Instant::now());
                current_task.step(&mut self.manager)?;

                // If an I/O request has been made, transition and block task.
                if !current_task.context().block_requests().is_empty() {
                    *current_task.context_mut().state_mut() = TaskStatus::Blocked;
                    blocked_tasks.push(current_task);
                } else if *current_task.context().program_counter() >= current_task.steps().len() {
                    // If the task has reached the end, check whether it is allowed to run again.
                    match current_task.shots() {
                        Shot::Custom(0) => {
                            *current_task.context_mut().state_mut() = TaskStatus::Exited
                        }
                        Shot::Infinity | Shot::Custom(_) => ready_tasks.push(current_task),
                    }
                } else {
                    ready_tasks.push(current_task);
                }
            }

            // Check how many blocked tasks are now free to be moved back into ready.
            let mut free_tasks: Vec<_> = blocked_tasks
                .extract_if(|blocked_task| blocked_task.context().block_requests().is_empty())
                .collect();

            // TODO: Move this into an entirely new state, the auxillary queue as in the book.
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

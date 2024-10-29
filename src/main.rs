use anyhow::Context;
use esp_idf_hal::gpio::*;

mod resource;
mod scheduler;
mod task;
mod util;

use resource::TaskResource;
use scheduler::TaskScheduler;
use task::{Shot, Task, TaskPriority, TaskStep};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut scheduler = TaskScheduler::new().context("Could not start task scheduler!")?;

    let task = Task::new(
        "blink",
        TaskPriority::Low,
        Shot::Infinity,
        vec![
            TaskStep::WriteGPIO(2, Level::High),
            TaskStep::Yield(1000),
            TaskStep::WriteGPIO(2, Level::Low),
            TaskStep::Yield(1000),
        ],
    )
    .assign(TaskResource::Pin(2));

    scheduler.add(task);

    scheduler.run()
}

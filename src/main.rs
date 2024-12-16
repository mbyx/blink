#![feature(map_many_mut, extract_if)]

use anyhow::Context;
use esp_idf_hal::gpio::*;

mod resource;
mod scheduler;
mod task;

use resource::TaskResource;
use scheduler::TaskScheduler;
use task::{Shot, Task, TaskContext, TaskPriority, TaskStep};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut scheduler = TaskScheduler::new().context("Could not start task scheduler!")?;

    let blink_always = Task::builder()
        .shots(Shot::Infinity)
        .context(
            TaskContext::builder()
                .name("Blink Blue LED")
                .priority(TaskPriority::Low)
                .build(),
        )
        .steps(vec![
            TaskStep::WriteGPIO(2, Level::High),
            TaskStep::Yield(1000),
            TaskStep::WriteGPIO(2, Level::Low),
            TaskStep::Yield(1000),
        ])
        .build()
        .assign(TaskResource::Pin(2));

    let blink_five_times = Task::builder()
        .shots(Shot::Custom(5))
        .context(
            TaskContext::builder()
                .name("Blink Green LED")
                .priority(TaskPriority::Low)
                .build(),
        )
        .steps(vec![
            TaskStep::Yield(1000),
            TaskStep::WriteGPIO(4, Level::High),
            TaskStep::Yield(1000),
            TaskStep::WriteGPIO(4, Level::Low),
        ])
        .build()
        .assign(TaskResource::Pin(4));

    scheduler.schedule_bulk(vec![blink_always, blink_five_times]);

    scheduler.run()
}

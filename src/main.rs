#![feature(map_many_mut, extract_if)]

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

    // let _ = util::escape_watchdog();

    let mut scheduler = TaskScheduler::new().context("Could not start task scheduler!")?;

    let blink_1 = Task::new(
        "Blink Blue LED",
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

    let blink_2 = Task::new(
        "Blink Green LED",
        TaskPriority::Low,
        Shot::Custom(5),
        vec![
            TaskStep::Yield(1000),
            TaskStep::WriteGPIO(4, Level::High),
            TaskStep::Yield(1000),
            TaskStep::WriteGPIO(4, Level::Low),
        ],
    )
    .assign(TaskResource::Pin(4));

    scheduler.schedule_bulk(vec![blink_1, blink_2]);

    scheduler.run()?;

    Ok(())
}

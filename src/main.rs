#![feature(map_many_mut, extract_if)]

use anyhow::Context;
use esp_idf_hal::{cpu, gpio::*, sys::{esp_task_wdt_delete, xTaskGetIdleTaskHandleForCore}, task::watchdog};

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

    // https://github.com/esp-rs/esp-idf-hal/issues/124
    unsafe { esp_task_wdt_delete(xTaskGetIdleTaskHandleForCore(cpu::core() as i32)) };

    let mut scheduler = TaskScheduler::new().context("Could not start task scheduler!")?;

    let task = Task::new(
        "blink blue",
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

    let task = Task::new(
        "blink red",
        TaskPriority::Low,
        Shot::Infinity,
        vec![
            TaskStep::Yield(1000),
            TaskStep::WriteGPIO(4, Level::High),
            TaskStep::Yield(1000),
            TaskStep::WriteGPIO(4, Level::Low),
        ],
    )
    .assign(TaskResource::Pin(4));

    scheduler.add(task);


    
    scheduler.run()?;

    Ok(())
}

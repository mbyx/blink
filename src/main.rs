#![allow(dead_code)]

use esp_idf_hal::gpio::*;

mod resource;
mod task;

use resource::{ResourceManager, TaskResource};
use task::{Task, TaskPriority, TaskStep};

// TODO: Make it work with the way esp-hal works.
// GPIO pins must only be referenced once or something. Probably force the scheduler to handle pin allocation,
// similar to file allocation.

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut manager = ResourceManager::new()?;

    let mut task = Task::new(
        "blink",
        TaskPriority::Low,
        vec![
            TaskStep::WriteGPIO(2, Level::High),
            TaskStep::Delay(1000),
            TaskStep::WriteGPIO(2, Level::Low),
            TaskStep::Delay(1000),
        ],
    )
    .acquire_resource(TaskResource::Pin(1));

    loop {
        task.run(&mut manager)?;
    }
}

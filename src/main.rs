#![allow(dead_code)]

use esp_idf_hal::gpio::*;

mod task;

use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::prelude::Peripherals;
use task::priority::TaskPriority;
use task::step::TaskStep;
use task::task::Task;

// TODO: Make it work with the way esp-hal works.
// GPIO pins must only be referenced once or something. Probably force the scheduler to handle pin allocation,
// similar to file allocation.

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

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
    .acquire_resource(task::resource::TaskResource::Pin(
        peripherals.pins.gpio2.downgrade().into_ref(),
    ));

    loop {
        task.run()?;
    }
}

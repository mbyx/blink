use std::mem::ManuallyDrop;

use crate::resource::{Request, ResourceManager, TaskResource};
use crate::util;

use super::TaskContext;
use anyhow::Context;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Level, PinDriver};

/// Represents the smallest instruction that a task can perform.
///
/// A task is made up of a series of steps, which can be one of these
/// atomic operations, which can include IO operations such as reading,
/// writing, logging, as well as yield control back to the scheduler.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TaskStep {
    /// Read the output from a GPIO pin and store it in a register.
    ReadGPIO(i32),
    /// Write to an input GPIO pin the given level or state.
    WriteGPIO(i32, Level),
    /// Yield execution back to the scheduler.
    Yield(u32),
    // TODO: Allow the usage of analog and spi pins.
    // TODO: Add more operations to handle files and logging.
}

impl<'a> TaskStep {
    /// Execute a single step of a task with the given context and acquired resources.
    pub fn execute(
        &mut self,
        context: &mut TaskContext,
        manager: &mut ResourceManager<'a>,
    ) -> anyhow::Result<Option<Request>> {
        match self {
            Self::ReadGPIO(pin_number) => {
                let pin = manager.acquire(TaskResource::Pin(*pin_number), context)?;
                let driver = ManuallyDrop::new(
                    PinDriver::input(pin.reborrow())
                        .context("This error is not possible as the driver is only used once before being wiped.")?
                );

                log::info!(
                    "Reading GPIO Pin {} Resulted In The Following Output: {:?}",
                    driver.pin(),
                    driver.get_level()
                );
            }
            Self::WriteGPIO(pin_number, level) => {
                let pin = manager.acquire(TaskResource::Pin(*pin_number), context)?;
                let mut driver = ManuallyDrop::new(
                    PinDriver::output(pin.reborrow())
                        .context("This error is not possible as the driver is only used once before being wiped.")?
                );
                driver.set_level(*level)?;
            }
            Self::Yield(ms) => {
                return Ok(Some(Request::Yield(*ms as _)));
            }
        }

        Ok(None)
    }
}

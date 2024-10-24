use std::mem::ManuallyDrop;
use std::ops::Not;

use crate::resource::manager::ResourceManager;

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
    // TODO: Allow the usage of analog and spi pins.
    /// Write to an input GPIO pin the given level or state.
    WriteGPIO(i32, Level),
    /// Yield execution back to the scheduler.
    Delay(u32),
    // TODO: Add more operations to handle files and logging.
}

impl TaskStep {
    /// Execute a single step of a task with the given context and acquired resources.
    pub fn execute(
        &mut self,
        context: &mut TaskContext,
        manager: &mut ResourceManager,
    ) -> anyhow::Result<()> {
        match self {
            Self::ReadGPIO(pin_number) => {
                // TODO: Couple this line of code and 'find_pin' so that we never forget to check
                // TODO: if the pin was assigned.
                context
                    .pins()
                    .contains(pin_number)
                    .not()
                    .then_some(())
                    .context("Unable to use pins that weren't acquired.")?;

                let pin = manager.find_pin(*pin_number)?;

                // TODO: Consider implications of a memory leak as well as in general creation
                // TODO: of a new 'PinDriver' every step, which may cause lag and other problems.
                let driver = ManuallyDrop::new(PinDriver::input(pin.reborrow())?);
                log::info!(
                    "Reading GPIO Pin {} Resulted In The Following Output: {:?}",
                    driver.pin(),
                    driver.get_level()
                );
            }
            Self::WriteGPIO(pin_number, level) => {
                // TODO: Couple this line of code and 'find_pin' so that we never forget to check
                // TODO: if the pin was assigned.
                context
                    .pins()
                    .contains(pin_number)
                    .then_some(())
                    .context("Unable to use pins that weren't acquired.")?;

                let pin = manager.find_pin(*pin_number)?;
                // TODO: Consider implications of a memory leak as well as in general creation
                // TODO: of a new 'PinDriver' every step, which may cause lag and other problems.
                let mut driver = ManuallyDrop::new(PinDriver::output(pin.reborrow())?);
                driver.set_level(*level)?;
            }
            Self::Delay(ms) => {
                FreeRtos::delay_ms(*ms);
            }
        }

        Ok(())
    }
}

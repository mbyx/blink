use std::mem::ManuallyDrop;
use std::ops::Not;

use crate::resource::manager::ResourceManager;

use super::context::TaskContext;
use anyhow::Context;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Level, PinDriver};

pub enum TaskStep {
    ReadGPIO(i32),
    WriteGPIO(i32, Level),
    Delay(u32),
}

impl TaskStep {
    pub fn execute(&mut self, context: &mut TaskContext, manager: &mut ResourceManager) -> anyhow::Result<()> {
        match self {
            Self::ReadGPIO(pin_number) => {
                context.pins()
                    .contains(pin_number)
                    .not()
                    .then_some(())
                    .context("Unable to use pins that weren't acquired.")?;
                
                let pin = manager.find_pin(*pin_number)?;
                // This part definitely causes a memory leak, but that's future me's problem.
                let driver = ManuallyDrop::new(PinDriver::input(pin.reborrow())?);
                log::info!(
                    "Reading GPIO Pin {} Resulted In The Following Output: {:?}",
                    driver.pin(),
                    driver.get_level()
                );
            }
            Self::WriteGPIO(pin_number, level) => {
                context.pins()
                    .contains(pin_number)
                    .then_some(())
                    .context("Unable to use pins that weren't acquired.")?;

                let pin = manager.find_pin(*pin_number)?;
                // This part definitely causes a memory leak, but that's future me's problem.
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

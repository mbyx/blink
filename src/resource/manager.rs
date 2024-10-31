use anyhow::Context;
use esp_idf_hal::{gpio::AnyIOPin, peripheral::PeripheralRef, prelude::Peripherals};

use crate::{task::TaskContext, util};

use super::TaskResource;

/// An asset manager but for IO resources.
///
/// This structure is the only source of truth for access and control of
/// any IO device or resource. It is unsafe to access a resource using a way
/// other than the 'ResourceManager', and it will lead to undefined behaviour.
pub struct ResourceManager<'a> {
    /// A list of peripherals or pins that it will manage.
    peripherals: Peripherals,
    /// A ready to access list of references to GPIO pins.
    pins: Vec<PeripheralRef<'a, AnyIOPin>>,

    // TODO: Add support for resources other than pins.
}

impl<'a> ResourceManager<'a> {
    /// Create a new resource manager.
    /// 
    /// This will setup all the references it needs to all the peripheral devices that
    /// it manages. Additional pins can also be accessed from it.
    pub fn new() -> anyhow::Result<Self> {
        let mut peripherals = Peripherals::take()
            .context("This should never occur as `take` is only called once.")?;

        // Safety: This operation is safe as only this struct will ever have access
        // to these resources at all times.
        let pins = util::pref! {
            peripherals.pins.gpio0,
            peripherals.pins.gpio1,
            peripherals.pins.gpio2,
            peripherals.pins.gpio3,
            peripherals.pins.gpio4,
            peripherals.pins.gpio5,
            peripherals.pins.gpio6,
            peripherals.pins.gpio7,
            peripherals.pins.gpio8,
            peripherals.pins.gpio9,
            peripherals.pins.gpio10,
            peripherals.pins.gpio11,
            peripherals.pins.gpio12,
            peripherals.pins.gpio13,
            peripherals.pins.gpio14,
            peripherals.pins.gpio15,
            peripherals.pins.gpio16,
            peripherals.pins.gpio17,
            peripherals.pins.gpio18,
            peripherals.pins.gpio19,
            peripherals.pins.gpio20,
            peripherals.pins.gpio21
        };

        Ok(Self {
            peripherals,
            pins: pins,
        })
    }

    // TODO: Modify this function to return a ResourceRef instead.

    /// This method safely checks whether a task has declared a pin before giving
    /// access to it. This prevents two tasks from using the same pin at the same
    /// time which may lead to undefined behaviour.
    pub fn acquire(&mut self, resource: TaskResource, context: &mut TaskContext) -> anyhow::Result<&mut PeripheralRef<'a, AnyIOPin>> {
        match resource {
            TaskResource::Pin(number) => {
                context
                    .pins_used()
                    .contains(&number)
                    .then_some(())
                    .context("Cannot acquire pin that hasn't been declared!")?;

                self.pins
                    .get_mut(number as usize)
                    .context(format!("Pin {number} not found!"))
            }
        }
    }
}

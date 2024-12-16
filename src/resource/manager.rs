use anyhow::Context;
use esp_idf_hal::{
    gpio::AnyIOPin,
    peripheral::{Peripheral, PeripheralRef},
    prelude::Peripherals,
};

use crate::task::TaskContext;

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
    // TODO: Add support for keeping track of which task has what resource.
}

impl<'a> ResourceManager<'a> {
    /// Create a new resource manager.
    ///
    /// This will setup all the references it needs to all the peripheral devices that
    /// it manages. Additional pins can also be accessed from it.
    pub fn new() -> anyhow::Result<Self> {
        let mut peripherals =
            Peripherals::take().context("Peripherals have already been taken.")?;

        // TODO: Consider creating a more generic version of this.
        // Safety: This operation is safe as only this struct will ever have access
        // to these resources at all times.
        let pins = util::pref!(pins: 0..21);

        Ok(Self { peripherals, pins })
    }

    // TODO: Modify this function to return a ResourceRef instead.
    // A ResourceRef is similar to a PeripheralRef however is generic over all
    // resources, and provides a simpler API.

    /// This method safely checks whether a task has declared a pin before giving
    /// access to it. This prevents two tasks from using the same pin at the same
    /// time which may lead to undefined behaviour.
    pub fn acquire(
        &mut self,
        resource: TaskResource,
        context: &mut TaskContext,
    ) -> anyhow::Result<&mut PeripheralRef<'a, AnyIOPin>> {
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

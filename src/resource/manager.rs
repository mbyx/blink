use anyhow::Context;
use esp_idf_hal::{
    gpio::{AnyIOPin, IOPin},
    peripheral::{Peripheral, PeripheralRef},
    prelude::Peripherals,
};

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
    pub fn new() -> anyhow::Result<Self> {
        // TODO: Check if this can be written in a more ergonomic way.
        let mut peripherals = Peripherals::take()?;
        // Safety: This operation is safe because the goal is to NEVER let anything other than ResourceManager handle these.
        let pin0 = unsafe { peripherals.pins.gpio0.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin1 = unsafe { peripherals.pins.gpio1.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin2 = unsafe { peripherals.pins.gpio2.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin3 = unsafe { peripherals.pins.gpio3.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin4 = unsafe { peripherals.pins.gpio4.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin5 = unsafe { peripherals.pins.gpio5.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin6 = unsafe { peripherals.pins.gpio6.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin7 = unsafe { peripherals.pins.gpio7.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin8 = unsafe { peripherals.pins.gpio8.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin9 = unsafe { peripherals.pins.gpio9.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin10 = unsafe { peripherals.pins.gpio10.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin11 = unsafe { peripherals.pins.gpio11.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin12 = unsafe { peripherals.pins.gpio12.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin13 = unsafe { peripherals.pins.gpio13.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin14 = unsafe { peripherals.pins.gpio14.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin15 = unsafe { peripherals.pins.gpio15.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin16 = unsafe { peripherals.pins.gpio16.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin17 = unsafe { peripherals.pins.gpio17.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin18 = unsafe { peripherals.pins.gpio18.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin19 = unsafe { peripherals.pins.gpio19.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin20 = unsafe { peripherals.pins.gpio20.clone_unchecked() }
            .downgrade()
            .into_ref();
        let pin21 = unsafe { peripherals.pins.gpio21.clone_unchecked() }
            .downgrade()
            .into_ref();

        let mut manager = Self {
            peripherals,
            pins: vec![],
        };
        manager.pins = vec![
            pin0, pin1, pin2, pin3, pin4, pin5, pin6, pin7, pin8, pin9, pin10, pin11, pin12, pin13,
            pin14, pin15, pin16, pin17, pin18, pin19, pin20, pin21,
        ];
        Ok(manager)
    }

    pub fn find_pin(&mut self, number: i32) -> anyhow::Result<&mut PeripheralRef<'a, AnyIOPin>> {
        self.pins.get_mut(number as usize).context("Pin not found!")
    }
}

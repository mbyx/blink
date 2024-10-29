/// Create a PeripheralReference to a pin or resource managed by the Peripherals struct.
///
/// This macro is unsafe, without the condition that only one copy of this reference is used
/// by the user, as otherwise any drivers created using that pin or resource will not work
/// correctly.

macro_rules! pref {
    // Take in a list of expressions, which are some kind of peripheral, and
    // create an unsafe reference to them.
    ($($peripheral:expr),+) => {{
            use esp_idf_hal::{peripheral::Peripheral, gpio::IOPin};
            let mut peripherals = vec![];
            $(
                peripherals.push(unsafe { $peripheral.clone_unchecked() }
                    .downgrade()
                    .into_ref());
            )+
            peripherals
    }};
}

pub(crate) use pref;

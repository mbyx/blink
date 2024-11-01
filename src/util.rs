use anyhow::Context;
use esp_idf_hal::{
    cpu,
    sys::{esp_task_wdt_delete, xTaskGetCurrentTaskHandleForCore, EspError},
};

// TODO: Instead of deleting the timer, simply ping it in a safe way.

/// Remove this program from the watchdog timer.
///
/// This program needs to run of extended periods of time in order to schedule tasks,
/// however the watchdog expects it to ping every few milliseconds to keep it alive.
/// By disabling the watchdog timer for this program, there is not need to do this.
pub fn escape_watchdog() -> anyhow::Result<()> {
    let result =
        unsafe { esp_task_wdt_delete(xTaskGetCurrentTaskHandleForCore(cpu::core() as i32)) };

    EspError::from(result)
        .map(|e| Err(e).context("Could not delete watchdog timer!"))
        .unwrap_or(Ok(()))
}

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

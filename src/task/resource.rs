use esp_idf_hal::{gpio::AnyIOPin, peripheral::PeripheralRef};

// Consider taking the pin driver instead. Prefer probably.
pub enum TaskResource<'a> {
    Pin(PeripheralRef<'a, AnyIOPin>),
}

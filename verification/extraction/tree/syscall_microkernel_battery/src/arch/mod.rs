// NONOS Operating System (AGPL-3.0-or-later)
//! What the battery call asks of the firmware: whether its ACPI namespace
//! declares a battery. That answer is the firmware's, not the kernel's, so it
//! is modelled here as one the kernel does not have yet (no table parsed);
//! the call's branches on it are the kernel's own code, included unchanged.

pub mod x86_64 {
    pub mod acpi {
        pub mod parser {
            pub struct Data {
                pub power_devices: PowerDevices,
            }

            pub struct PowerDevices {
                pub battery: bool,
            }

            pub fn with_data<R>(_f: impl FnOnce(&Data) -> R) -> Option<R> {
                None
            }
        }
    }
}

// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel starts this driver for a machine whose PCI scan finds an
//! LPSS I2C function (src/hardware/inventory/i2c_host.rs); every id the
//! driver itself takes is one the kernel's list names, so no listed
//! controller waits on a spawn that never comes.

// The kernel list, mounted once in discover.rs (a second mount is clippy::duplicate_mod).
use crate::discover::kernel_i2c_host as kernel;

use crate::constants::device_info;

#[test]
fn every_i2c_function_the_driver_takes_is_one_the_kernel_spawns_for() {
    // A class that is neither serial bus nor signal processing: the id alone.
    let listed = |id| kernel::i2c_host(1, 0x8086, id, 0, (0x00, 0x00));
    for id in 0u16..=0xFFFF {
        if device_info(id).is_some() {
            assert!(listed(id), "8086:{id:04x} is taken by the driver, not spawned for");
        }
    }
}

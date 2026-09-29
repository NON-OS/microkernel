// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../src/hardware/broker/pio_absent.rs"]
pub mod pio_absent;

pub use pio_absent::{pio_release_all_for_pid, pio_release_for_device};

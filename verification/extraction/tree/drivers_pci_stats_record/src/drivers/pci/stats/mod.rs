// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/drivers/pci/stats/atomics.rs"]
pub mod atomics;

#[path = "../../../../../../../../src/drivers/pci/stats/record.rs"]
pub mod record;

pub use record::{record_config_error, record_config_read, record_config_write, record_device_found, record_enumeration, record_error_event, record_hotplug_event, record_interrupt, record_link_state_change, record_power_state_change, reset_stats};

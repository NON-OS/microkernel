// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the WiFi settings panel. The real panel source is included
//! via #[path] and exercised against synthetic inputs: adapter discovery, the
//! untrusted scan-result parser, and the scans and joins the panel's ticks
//! run.

extern crate alloc;

#[path = "../../capsule_settings/src/wifi/adapter_name.rs"]
pub mod adapter_name;
#[path = "../../capsule_settings/src/wifi/driver_ids.rs"]
pub mod driver_ids;
#[path = "../../capsule_settings/src/wifi/interface.rs"]
pub mod interface;
/// The iwlwifi driver's own id table, so the panel's has-a-driver rule is
/// held to it.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../capsule_driver_iwlwifi/src/firmware/family.rs"]
mod iwlwifi_family;
#[path = "../../capsule_settings/src/wifi/wifi_class.rs"]
pub mod wifi_class;

/* How often the Wi-Fi and Network pages ask the DHCP client for its lease. */
#[path = "../../capsule_settings/src/wifi/net_poll.rs"]
pub mod net_poll;

/// The pure WiFi panel modules, grouped so their `super::` references resolve;
/// adapter enumeration calls the broker and stays out.
pub mod wifi;

/// The policy-call error and the pass that reads every stored value when the
/// panel opens; the calls it makes are syscalls and stay out.
pub mod ipc;

// The panel's sections and row tables, as the capsule has them.
#[path = "settings_schema.rs"]
pub mod settings;

// The Bluetooth page's logic, apart from its calls to driver.bt_usb0.
pub mod bluetooth;

// The notes under the rows a person sets, Sound's among them.
#[path = "../../capsule_settings/src/settings/ui/field_note_user.rs"]
pub mod field_note_user;

/// The text buffers the panel's fields type into, a character at a time.
pub mod text;

/// The answer a Wi-Fi worker thread hands back to the window thread, as libc
/// keeps it for every app with a worker.
#[path = "../../capsule_settings/src/settings/state/wifi/cursor.rs"]
pub mod wifi_cursor;

#[path = "../../libc/src/thread/handoff.rs"]
pub mod handoff;

/// The saved-network record from `nonos_wifi_client`, driven with keys the
/// tests choose; its vfs and TPM halves are syscalls and stay out.
#[cfg(test)]
mod saved;
#[cfg(test)]
use wifi::{join_wire, network};
#[cfg(test)]
#[path = "../../nonos_wifi_client/src/wipe.rs"]
mod wipe;

/* The Qwen model row's tiers, as Settings steps through them. */
#[path = "../../capsule_settings/src/settings/qwen_tier/mod.rs"]
pub mod qwen_tier;

#[cfg(test)]
mod bt_tests;
#[cfg(test)]
pub mod tests;

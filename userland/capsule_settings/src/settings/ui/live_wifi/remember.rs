// NONOS Operating System (AGPL-3.0-or-later)

//! The remember row, and what the panel says with no driver.

use super::super::valbuf::ValBuf;
use crate::settings::schema::rows::Tone;
use crate::settings::state::State;
use crate::wifi::has_driver;

/// Remembering is offered only on a boot that keeps state.
pub fn remember(b: &mut ValBuf, state: &State) -> Tone {
    match (state.wifi.keeps, state.wifi.remember) {
        (false, _) => b.push_str("Off: this boot keeps nothing"),
        (true, true) => b.push_str("On"),
        (true, false) => b.push_str("Off"),
    }
    Tone::Idle
}

/*
 * No Wi-Fi driver answers. Say why from the chip the bus lists: none at all; a
 * chip this build has a driver for that is not running (it did not spawn, or
 * spawned and left); or a chip with no NONOS driver, which no wait will fix.
 * The row holds 72 bytes, so each line fits with the ids.
 */
pub(super) fn no_driver(b: &mut ValBuf, state: &State) -> Tone {
    if state.wifi_adapter_count == 0 {
        b.push_str("No Wi-Fi hardware found");
        return Tone::Idle;
    }
    let (vendor, device) = state.wifi_adapters[0].ids();
    if has_driver(vendor, device) {
        b.push_str("Wi-Fi driver did not start for ");
        push_ids(b, vendor, device);
    } else {
        b.push_str("Wi-Fi chip ");
        push_ids(b, vendor, device);
        b.push_str(" has no NONOS driver; use Ethernet or USB Wi-Fi");
    }
    Tone::Warn
}

fn push_ids(b: &mut ValBuf, vendor: u16, device: u16) {
    b.push_hex16(vendor);
    b.push_str(":");
    b.push_hex16(device);
}

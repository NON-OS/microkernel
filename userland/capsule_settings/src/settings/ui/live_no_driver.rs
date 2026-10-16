/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The link row with no Wi-Fi driver answering: why, from the chip the bus
//! lists, with its vendor and device ids.

use crate::settings::schema::rows::Tone;
use crate::settings::state::State;
use crate::wifi::has_driver;

use super::valbuf::ValBuf;

/*
 * No Wi-Fi driver answers. Say why from the chip the bus lists: none at all; a
 * chip this build has a driver for that is not running (it did not spawn, or
 * spawned and left); or a chip with no NONOS driver, which no wait will fix.
 * The row holds 72 bytes, so each line fits with the ids.
 */
pub fn no_driver(b: &mut ValBuf, state: &State) -> Tone {
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

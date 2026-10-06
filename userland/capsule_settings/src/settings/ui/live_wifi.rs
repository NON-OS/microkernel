/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi page's live rows: the link, the join in progress or its outcome,
//! and whether joins are remembered.

use nonos_wifi_client::{firmware_step_text, join_text, DriverStage};

use crate::settings::schema::rows::Tone;
use crate::settings::state::wifi_pending::Work;
use crate::settings::state::{State, WifiConnect};
use crate::wifi::has_driver;

use super::valbuf::ValBuf;

const PASS_STARS: usize = 32;

/// Which driver runs and where its link stands.
pub fn link(b: &mut ValBuf, state: &State) -> Tone {
    let Some(driver) = state.wifi.driver else {
        return no_driver(b, state);
    };
    if let Some(ssid) = state.wifi.joined() {
        b.push_str("Connected to ");
        b.push_bytes(ssid);
        return Tone::Ok;
    }
    if state.wifi_stage == Some(DriverStage::Ready) {
        b.push_str("Not connected");
        return Tone::Idle;
    }
    // A firmware failure names its step and the control register's bits in
    // place of the stage line, so a machine with no serial port still says
    // which part of the load stopped. The line holds 72 bytes: the longest
    // step text is 41, so the adapter name gives way to it.
    if state.wifi_stage == Some(DriverStage::FirmwareFailed) {
        let why = state.wifi_datapath.and_then(|dp| Some((firmware_step_text(dp.fw_step)?, dp)));
        if let Some((why, dp)) = why {
            b.push_str("Firmware stopped: ");
            b.push_str(why);
            b.push_str(" (0x");
            b.push_hex16(dp.fw_ctrl as u16);
            b.push_str(")");
            return Tone::Warn;
        }
    }
    b.push_str(driver.label());
    b.push_str(": ");
    b.push_str(state.wifi_stage.map_or("not answering", |s| s.text()));
    Tone::Warn
}

/// The passphrase being typed, masked; else what the last action did.
pub fn join(b: &mut ValBuf, state: &State) -> Tone {
    if state.wifi_pass_active {
        // Typed stars stay inside the 72-byte line with the hint after them.
        b.push_str("Passphrase: ");
        let typed = &state.wifi_pass.as_slice()[..state.wifi_pass.len.min(PASS_STARS)];
        if state.wifi_pass_shown {
            b.push_bytes(typed);
        } else {
            for _ in typed {
                b.push_str("*");
            }
        }
        b.push_str("_  Tab shows, Enter joins");
        return Tone::Warn;
    }
    match state.wifi.pending.work() {
        Some(Work::Scan) => {
            b.push_str("Scanning...");
            return Tone::Idle;
        }
        Some(Work::Join) => {
            b.push_str("Joining ");
            b.push_bytes(state.wifi.join_net.ssid());
            b.push_str("...");
            return Tone::Idle;
        }
        None => {}
    }
    if let Some(notice) = state.wifi.notice {
        b.push_str(notice);
        return Tone::Idle;
    }
    match state.wifi_connect {
        WifiConnect::Idle => b.push_str("--"),
        WifiConnect::Connected => b.push_str(join_text(0)),
        WifiConnect::Failed(r) => {
            b.push_str(join_text(r.code));
            return Tone::Warn;
        }
    }
    Tone::Idle
}

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
fn no_driver(b: &mut ValBuf, state: &State) -> Tone {
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

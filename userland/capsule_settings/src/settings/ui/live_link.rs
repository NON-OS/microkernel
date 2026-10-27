/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi page's link row: which driver runs and where its link stands,
//! or, with no driver answering, why not (`live_no_driver.rs`).

use nonos_wifi_client::{firmware_step_text, DriverStage, StopWhy};

use crate::settings::schema::rows::Tone;
use crate::settings::state::State;

use super::live_no_driver::no_driver;
use super::valbuf::ValBuf;

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
    if state.wifi_diag.stage == Some(DriverStage::Ready) {
        b.push_str("Not connected");
        return Tone::Idle;
    }
    // A firmware failure names its step and the control register's bits in
    // place of the stage line, so a machine with no serial port still says
    // which part of the load stopped. The line holds 72 bytes: the longest
    // step text is 41, so the adapter name gives way to it.
    if state.wifi_diag.stage == Some(DriverStage::FirmwareFailed) {
        let why =
            state.wifi_diag.datapath.and_then(|dp| Some((firmware_step_text(dp.fw_step)?, dp)));
        if let Some((why, dp)) = why {
            b.push_str("Firmware stopped: ");
            b.push_str(why);
            b.push_str(" (0x");
            b.push_hex16(dp.fw_ctrl as u16);
            b.push_str(")");
            return Tone::Warn;
        }
    }
    // A refused claim names the step and the broker's errno (`StopWhy`): the
    // longest line, "Card not claimed: memory enable refused, device
    // resetting (-11)", is 63 bytes.
    if state.wifi_diag.stage == Some(DriverStage::NotClaimed) {
        let mut buf = [0u8; 64];
        let claim = state.wifi_diag.datapath.map(|d| StopWhy::Claim(d.claim_step, d.claim_errno));
        if let Some(why) = claim.and_then(|c| c.text(&mut buf)) {
            b.push_str("Card ");
            b.push_bytes(why);
            return Tone::Warn;
        }
    }
    b.push_str(driver.label());
    b.push_str(": ");
    b.push_str(state.wifi_diag.stage.map_or("not answering", |s| s.text()));
    Tone::Warn
}

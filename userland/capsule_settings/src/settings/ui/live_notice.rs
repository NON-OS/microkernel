/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! What the last remember or forget did, on the Wi-Fi page's join row: a
//! remember that dropped a network from a full list names it (W2), here
//! and nowhere else.

use crate::settings::state::wifi::REMEMBERED;
use crate::settings::state::State;

use super::super::valbuf::ValBuf;

/// `notice`, or for the remember that dropped a network, that network by
/// name; 19 + 32 + 13 bytes fit the row whole.
pub fn say(b: &mut ValBuf, state: &State, notice: &'static str) {
    match state.wifi.dropped.as_ref().filter(|_| notice == REMEMBERED) {
        Some(gone) => {
            b.push_str("Remembered; forgot ");
            b.push_bytes(gone.ssid());
            b.push_str(" to make room");
        }
        None => b.push_str(notice),
    }
}

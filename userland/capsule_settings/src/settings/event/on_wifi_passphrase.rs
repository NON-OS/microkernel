/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Typing a Wi-Fi passphrase. The characters are held only in the panel's
//! edit buffer, drawn masked, and wiped on join or on Escape.

use nonos_app_skeleton::{EventOutcome, KEY_ENTER, KEY_ESC, KEY_TAB};

use crate::settings::state::wifi_join::{clear_passphrase, connect_selected};
use crate::settings::state::State;

use super::on_event_wifi::repaint_after;

const KEY_BACKSPACE: u32 = 0x08;
const KEY_DELETE: u32 = 0x7F;

/// While the passphrase editor is open: type printable characters, delete with
/// backspace, join on Enter, cancel on Escape.
pub(super) fn on_passphrase_key(state: &mut State, code: u32) -> EventOutcome {
    match code {
        KEY_ESC => repaint_after(state, clear_passphrase),
        KEY_ENTER => repaint_after(state, connect_selected),
        // Show what was typed, or hide it again, so a capital or a symbol the
        // keyboard layout moved can be seen before joining.
        KEY_TAB => {
            state.wifi_pass_shown = !state.wifi_pass_shown;
            EventOutcome::Repaint
        }
        KEY_BACKSPACE | KEY_DELETE => {
            state.wifi_pass.pop();
            EventOutcome::Repaint
        }
        c @ 0x20..=0x7E => {
            state.wifi_pass.push(c as u8);
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}

// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The receive screen's words and QR sizes: each lead, and what shows
//! while there is no private address yet.

use crate::wallet::etna::tokens::{BAD, TEXT_3};
use crate::wallet::shield::probe::Shield;
use crate::wallet::state::State;

pub(super) const PUBLIC: &str = "Public: anyone can see what this address holds and sends.";
pub(super) const PUBLIC_HERE_ONLY: &str =
    "Public: anyone can see what this address holds and sends. \
     Receiving privately runs on Sepolia, where the shield pool is; switch networks in Settings.";
pub(super) const PRIVATE: &str =
    "Private: paid here, the sender, the amount, your balance and the \
     notes you hold stay out of sight.";
const OPENING: &str = "Your private address appears here once the shield has opened for \
     this account.";
const NO_SHIELD: &str = "No shield service is running on this machine, so there is no private \
     address yet. Receive at the 0x address meanwhile.";
pub(super) const LIVE: &str =
    "This machine runs live, so the shield keeps its notes in memory until \
     reboot. Your recovery words open the same address again on any machine.";

pub(super) const QR_SIDE: u32 = 200;
/* 177 modules for a nox1: two pixels a module at the least. */
pub(super) const QR_PRIVATE: u32 = 400;

/* While there is no private address: the failure the service gave, else
 * what it is doing and for how long, else why there is none. */
pub(super) fn waiting_text(state: &State) -> (alloc::string::String, u32) {
    use alloc::string::String;
    let ui = &state.shield_ui;
    match (&ui.failure, ui.waiting) {
        (Some(why), _) => (why.clone(), BAD),
        (None, Some(w)) => {
            let doing = crate::wallet::screen::shield::proving::doing(w.op);
            let mut text = String::new();
            text.extend(doing.chars().next().map(|c| c.to_ascii_uppercase()));
            text.push_str(&doing[1..]);
            text.push_str(&alloc::format!(
                " on this machine, {} min {:02} s so far.",
                ui.elapsed_s / 60,
                ui.elapsed_s % 60
            ));
            if let Some(phase) = &ui.phase {
                text.push_str(&alloc::format!(" Now: {phase}."));
            }
            text.push_str(crate::wallet::shield::open::opening_said(w.op, ui.elapsed_s));
            (text, TEXT_3)
        }
        (None, None) if crate::wallet::shield::open::retrying(state) => {
            (String::from(crate::wallet::shield::client::STARTING), TEXT_3)
        }
        (None, None) if state.shield == Shield::Absent => (String::from(NO_SHIELD), TEXT_3),
        (None, None) => (String::from(OPENING), TEXT_3),
    }
}

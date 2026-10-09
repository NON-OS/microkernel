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

/*
 * 03  RECEIVE, as the phone app has it: the private nox1 address first,
 * whose payments nobody can link to this account's balance or notes, and
 * the public 0x address one press away. The QR is drawn large enough for a
 * phone to read the 2,009 letters.
 */

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::qr::qr;
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::tokens::{BAD, GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::shield::probe::Shield;
use crate::wallet::state::State;

const PUBLIC: &str = "Public: anyone can see what this address holds and sends.";
const PUBLIC_HERE_ONLY: &str = "Public: anyone can see what this address holds and sends. \
     Receiving privately runs on Sepolia, where the shield pool is; switch networks in Settings.";
const PRIVATE: &str = "Private: paid here, the sender, the amount, your balance and the \
     notes you hold stay out of sight.";
const OPENING: &str = "Your private address appears here once the shield has opened for \
     this account.";
const NO_SHIELD: &str = "No shield service is running on this machine, so there is no private \
     address yet. Receive at the 0x address meanwhile.";
const LIVE: &str = "This machine runs live, so the shield keeps its notes in memory until \
     reboot. Your recovery words open the same address again on any machine.";
const QR_SIDE: u32 = 200;
/* 177 modules for a nox1: two pixels a module at the least. */
const QR_PRIVATE: u32 = 400;

pub fn receive(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    /* Private only where the shield pool runs, whichever way this screen
     * was reached: off Sepolia nothing of the shield is shown here. */
    let here = crate::wallet::shield::open::here();
    let private = state.receive_private && here;
    let nox1 = state.shield_ui.nox1.as_deref().filter(|_| private);
    let ready = state.address_ready && (!private || nox1.is_some());
    let other = if private { "Receive at the 0x address" } else { "Receive privately" };
    let can_switch = state.address_ready && here;
    let footer = [("Copy address", Weight::Primary, ready), (other, Weight::Secondary, can_switch)];
    let spec = FrameSpec {
        number: "03",
        title: "Receive",
        back: true,
        backdrop: Some(Backdrop::Receive),
        failure: None,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let lead = match (private, here) {
        (true, _) => PRIVATE,
        (false, true) => PUBLIC,
        (false, false) => PUBLIC_HERE_ONLY,
    };
    let mut y = c.y
        + wrapped(fb, c.x as i32, c.y as i32, c.w as i32, Role::Lead, lead, TEXT_3) as u32
        + GAP;
    match (private, nox1) {
        (true, Some(addr)) => {
            let side = QR_PRIVATE.min(c.w);
            if qr(fb, c.x, y, side, addr.as_bytes()) {
                y += side + GAP;
            }
            y += value_block(fb, c.x, y, c.w, addr) + GAP;
            if state.shield_ui.live {
                y += wrapped(fb, c.x as i32, y as i32, c.w as i32, Role::Lead, LIVE, TEXT_3) as u32
                    + GAP;
            }
        }
        (true, None) => {
            let (text, colour) = waiting_text(state);
            y += wrapped(fb, c.x as i32, y as i32, c.w as i32, Role::Lead, &text, colour) as u32
                + GAP;
        }
        (false, _) if state.address_ready => {
            let hex = super::receive_address::address_text(state);
            let uri = alloc::format!("ethereum:{hex}");
            if qr(fb, c.x, y, QR_SIDE, uri.as_bytes()) {
                y += QR_SIDE + GAP;
            }
            y += value_block(fb, c.x, y, c.w, &hex) + GAP;
        }
        _ => {}
    }
    y += super::receive_address::facts(state, fb, c, y);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    if let Some(back) = l.back {
        hits::put(Press::Back, back);
    }
    if ready {
        hits::put(Press::Footer(0), l.footer[0]);
    }
    if can_switch {
        hits::put(Press::Footer(1), l.footer[1]);
    }
}

/* While there is no private address: the failure the service gave, else
 * what it is doing and for how long, else why there is none. */
fn waiting_text(state: &State) -> (alloc::string::String, u32) {
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

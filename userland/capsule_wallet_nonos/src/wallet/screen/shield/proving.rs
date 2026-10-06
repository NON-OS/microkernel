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
 * 09  PROVING: the proof being built on this machine and how long it has
 * taken, then the spend's way to the chain: handed to a lander, waiting,
 * landed. When no lander takes it, the holder may settle it from their own
 * account, told first what that reveals, or take the notes back.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;
use shield_wire::*;

use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::tokens::{BAD, CYAN, GAP, LINE_2, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits;
use crate::wallet::state::shield_ui::ShieldUi;
use crate::wallet::state::State;

const IDLE: &str = "Nothing is being proved right now. A proof starts when you confirm a \
     private payment or a withdrawal, and its progress and time taken are shown while it runs.";
const PROVING: &str = "The proof is being built on this machine. Nothing has left it yet.";
const EARLIER: &str = "Earlier payments still on their way, each followed to the block:";

pub const STOP: u8 = 0;
pub const SETTLE: u8 = 1;
pub const TAKE_BACK: u8 = 2;
pub const SETTLE_EARLIER: u8 = 3;

/* The earliest kept spend its owner may settle now, by its number. */
pub fn earlier_to_settle(ui: &ShieldUi) -> Option<&str> {
    ui.earlier.iter().find(|s| s.self_settle && s.tx.is_none()).and_then(|s| s.id.as_deref())
}

pub fn doing(op: u16) -> &'static str {
    match op {
        OP_SEND => "proving a private payment",
        OP_WITHDRAW => "proving a withdrawal",
        OP_OPEN_WORDS | OP_OPEN_KEY => "opening the shield",
        OP_SYNC => "reading the pool",
        OP_REVIEW_SHIELD | OP_REVIEW_SELF_SETTLE => "checking with the pool",
        OP_QUOTE => "reading the fee",
        OP_CONFIRM => "sending",
        OP_FOLLOW => "looking for the payment on chain",
        OP_TAKE_BACK => "taking the notes back",
        _ => "working",
    }
}

/* The buttons this screen offers now, in footer order. */
pub fn buttons(ui: &ShieldUi) -> Vec<(&'static str, u8)> {
    if let Some(w) = ui.waiting {
        return if matches!(w.op, OP_SEND | OP_WITHDRAW) {
            Vec::from([("Stop", STOP)])
        } else {
            Vec::new()
        };
    }
    let mut out = Vec::new();
    if let Some(s) =
        ui.spend.as_ref().filter(|s| s.tx.is_none() && !s.state.starts_with("settling"))
    {
        if s.self_settle {
            out.push(("Settle it myself", SETTLE));
        }
        if !s.published || s.refusal.is_some() {
            out.push(("Take back", TAKE_BACK));
        }
    }
    if earlier_to_settle(ui).is_some() {
        out.push(("Settle an earlier one myself", SETTLE_EARLIER));
    }
    out.truncate(3);
    out
}

pub fn proving(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let ui = &state.shield_ui;
    let status = super::status::parts(state);
    let shown = buttons(ui);
    let footer: Vec<(&str, Weight, bool)> = shown
        .iter()
        .enumerate()
        .map(|(i, (t, _))| (*t, if i == 0 { Weight::Primary } else { Weight::Secondary }, true))
        .collect();
    let spec = FrameSpec {
        number: "09",
        title: "Proving",
        back: true,
        backdrop: Some(Backdrop::Proving),
        failure: super::absent::banner(state),
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let (x, w) = (c.x as i32, c.w as i32);
    let mut y = c.y;
    if let Some(job) = ui.waiting {
        if matches!(job.op, OP_SEND | OP_WITHDRAW) {
            y += wrapped(fb, x, y as i32, w, Role::Lead, PROVING, TEXT_3) as u32 + GAP;
        }
        y += fact(fb, c.x, y, c.w, "now", doing(job.op));
        let (m, s) = (ui.elapsed_s / 60, ui.elapsed_s % 60);
        y += fact(fb, c.x, y, c.w, "elapsed", &format!("{m} min {s:02} s"));
        if let Some(phase) = &ui.phase {
            y += fact(fb, c.x, y, c.w, "phase", phase);
        }
        if let Some(p) = ui.permille {
            fb.fill_round(c.x, y + GAP / 2, c.w, 6, 3, LINE_2);
            let lit = (c.w as u64 * p as u64 / 1000) as u32;
            fb.fill_round(c.x, y + GAP / 2, lit.max(1), 6, 3, CYAN);
            y += GAP + 6;
        }
        y += GAP;
    }
    match &ui.spend {
        Some(s) => {
            let what = super::history_labels::kind(s.kind);
            y += fact(
                fb,
                c.x,
                y,
                c.w,
                "payment",
                &format!("{what} {} {}", s.amount, super::consts::ticker(s.asset)),
            );
            y += fact(fb, c.x, y, c.w, "state", &s.state);
            if s.published {
                y += fact(fb, c.x, y, c.w, "since handed off", &format!("{} min", s.minutes));
            }
            if let Some(tx) = s.tx.as_ref().filter(|t| !t.is_empty()) {
                y += fact(fb, c.x, y, c.w, "transaction", &super::review_text::short(tx));
            }
            for weak in &s.weakened {
                y += wrapped(fb, x, y as i32, w, Role::Lead, weak, BAD) as u32;
            }
            if let Some(why) = &s.refusal {
                let text = format!("The lander said: {why}");
                y += wrapped(fb, x, y as i32, w, Role::Lead, &text, BAD) as u32;
            }
            if s.self_settle && s.tx.is_none() {
                y += GAP / 2;
                y += wrapped(fb, x, y as i32, w, Role::Lead, super::review_rows::LINKS, TEXT_3)
                    as u32;
            }
        }
        None if ui.waiting.is_none() && ui.earlier.is_empty() => {
            y += wrapped(fb, x, y as i32, w, Role::Lead, IDLE, TEXT_3) as u32;
        }
        None => {}
    }
    /* Every spend proved before the last that has not landed yet. */
    if !ui.earlier.is_empty() {
        y += GAP;
        y += wrapped(fb, x, y as i32, w, Role::Lead, EARLIER, TEXT_3) as u32;
    }
    for s in &ui.earlier {
        let what = if s.amount.is_empty() {
            String::from("an earlier payment")
        } else {
            let kind = super::history_labels::kind(s.kind);
            format!("{kind} {} {}", s.amount, super::consts::ticker(s.asset))
        };
        let minutes = if s.minutes > 0 { format!(", {} min", s.minutes) } else { String::new() };
        y += fact(fb, c.x, y, c.w, &what, &format!("{}{minutes}", s.state));
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    super::edges::put(state, &l);
    let on: Vec<bool> = shown.iter().map(|_| true).collect();
    super::edges::footer(&l, &on);
}

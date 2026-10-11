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

//! A made order's dispute or refund as rows (B8, v1.4.51): the refund
//! address it goes from, the dispute window's end, and the transaction.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::wallet::order::intake::Pending;
use crate::wallet::order::send_step::SendStep;
use crate::wallet::screen::receive_address::checksummed_text;

const HOUR: u64 = 3_600;

fn left(end: u64, now: u64) -> String {
    match end.saturating_sub(now) {
        0 => String::from("ended"),
        s if s < HOUR => format!("ends in {} minutes", s / 60),
        s => format!("ends in {} hours", s / HOUR),
    }
}

pub fn rows(p: &Pending) -> Vec<(&'static str, String)> {
    let Some(e) = p.escrow.as_ref() else { return Vec::new() };
    let mut out = Vec::from([("from", checksummed_text(&p.refund_to))]);
    if let Some(end) = e.window_end {
        out.push(("dispute window", left(end, nonos_tls::rtc_now())));
    }
    if let SendStep::Sent(h) = &e.step {
        let short: String = h.iter().take(8).map(|b| format!("{b:02x}")).collect();
        out.push(("transaction", format!("0x{short}…")));
    }
    out
}

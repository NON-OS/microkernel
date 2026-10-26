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

//! What a Gauntlet boot shows of the receipt job's rule, beside its receipt
//! (capsule_prove receipt_step.rs): before the ack, the same ask is answered
//! with the same bytes and any other ask is refused EBUSY; after it, a new job
//! asked for the same run finds its record gone (ENOENT). One log line:
//!
//!   [AGENTPROOF-RECEIPT] v=1 rule again=same other=-16 after-ack=-2

use alloc::string::String;
use alloc::vec;

use super::dump::say;
use super::receipt_call::{job, status};
use super::receipt_frame::{FRAME_MAX, RECEIPT};

pub fn before_ack(port: u32, body: &[u8], napr: &[u8], rx: &mut [u8]) -> String {
    let again = match status(port, RECEIPT, 3, body, rx) {
        Some((0, b)) if b == napr => "same".into(),
        Some((0, _)) => "different".into(),
        s => code(s.map(|(c, _)| c)),
    };
    let mut other = body.to_vec();
    if let Some(b) = other.last_mut() {
        *b ^= 1;
    }
    let other = code(status(port, RECEIPT, 4, &other, rx).map(|(c, _)| c));
    alloc::format!("again={again} other={other}")
}

pub fn after_ack(body: &[u8], rule: &str) {
    let mut rx = vec![0u8; FRAME_MAX];
    let after = match job() {
        Ok(port) => code(status(port, RECEIPT, 1, body, &mut rx).map(|(c, _)| c)),
        Err(_) => "no-job".into(),
    };
    say(&alloc::format!("[AGENTPROOF-RECEIPT] v=1 rule {rule} after-ack={after}\n"));
}

fn code(c: Option<i32>) -> String {
    c.map_or("none".into(), |c| alloc::format!("{c}"))
}

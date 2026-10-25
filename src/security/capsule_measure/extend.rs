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

//! One admitted capsule into PCR 14 and the kernel's log: once for each
//! distinct body this boot, and never past what a client replays.

use alloc::vec::Vec;

use super::log::{broke, LOG};
use super::{banks::banks, event};
use crate::security::tpm::{found, pcr_extend::pcr_extend};
use crate::smp::lock_responsive;

/// Well under the events a client replays, with the firmware's before them.
const MAX_CAPSULES: usize = 1024;

/// Measures one admitted capsule into PCR 14 and its event into the log.
pub fn admitted(cert: &[u8; 32], verifier: &[u8; 32], kind: u8) {
    let mut log = lock_responsive(&LOG);
    let body = event::body(cert, verifier, kind);
    if log.broken || log.bodies.contains(&body) {
        return;
    }
    if log.bodies.len() == MAX_CAPSULES {
        return broke(&mut log, b"[MEASURE] more capsules than a log records");
    }
    let Some((bytes, digests)) = banks().and_then(|b| event::event(&b, &body)) else {
        return broke(&mut log, b"[MEASURE] a log bank this kernel cannot hash");
    };
    let refs: Vec<(u16, &[u8])> = digests.iter().map(|(a, d)| (*a, d.as_slice())).collect();
    match pcr_extend(event::PCR_CAPSULE, &refs) {
        Ok(()) => {
            log.events.extend_from_slice(&bytes);
            log.bodies.push(body);
        }
        Err(_) if !found() => {}
        Err(_) => broke(&mut log, b"[MEASURE] PCR 14 extend failed"),
    }
}

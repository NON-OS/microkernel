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

//! What a written reset came to (L78). The function must answer as itself,
//! not with a retry status, and its marker must read clear; then what the
//! reset cleared is put back, decode last and only once its BARs stuck
//! (L62), Bus Master and INTx Disable left 0 for the next holder's driver.

use super::bits_rule::{devctl2_after, devctl_after, not_ready, BME, INTX_DISABLE, IO, MEM};
use super::end_rule::{checked, End, Next};
use super::marker_rule::marker_bit;
use super::method_rule::fallback;
use super::table::Wipe;
use crate::drivers::pci::ConfigSpace;

pub(super) fn check(w: Wipe, now: u64) -> (Wipe, Option<End>) {
    let cfg = ConfigSpace::new(w.address);
    let id = cfg.read32(0x00).unwrap_or(u32::MAX);
    let ready = !not_ready(id) && id == w.caps.ident;
    let clear = |bit: u16| cfg.read16(0x04).is_ok_and(|v| v & bit == 0);
    let confirmed = ready && w.marker.map(marker_bit).is_some_and(clear);
    let fall = w.method.and_then(|m| fallback(m, w.caps.offers));
    match checked(w.phase, now, ready, confirmed, fall) {
        Next::Stay => (w, None),
        Next::Retry(m) => (super::reset::write(w, m, now), None),
        Next::Ended(End::Clean) => {
            put_back(&cfg, &w);
            (w, Some(End::Clean))
        }
        Next::Ended(end) => (w, Some(end)),
    }
}

fn put_back(cfg: &ConfigSpace, w: &Wipe) {
    let Some(s) = w.saved else { return };
    let stuck = super::super::restore::restore(w.device_id);
    let _ = cfg.write8(0x0C, s.cls);
    let _ = cfg.write8(0x0D, s.lt);
    let _ = cfg.write8(0x3C, s.int_line);
    let devctl = u16::from(w.caps.pcie) + 0x08;
    if let Some(d) = s.devctl {
        let _ = cfg.write16(devctl, devctl_after(d));
    }
    if let Some(d) = s.devctl2 {
        let _ = cfg.write16(devctl + 0x20, devctl2_after(d, w.caps.devctl2_boot));
    }
    if stuck {
        let keep = |v: u16| (v & !(IO | MEM | BME | INTX_DISABLE)) | (s.command & (IO | MEM));
        let _ = cfg.update_command(keep);
    }
}

/// A marker that survived goes back as it was before the wipe.
pub(super) fn unmark(cfg: &ConfigSpace, w: &Wipe) {
    let (Some(s), Some(m)) = (w.saved, w.marker) else { return };
    let bit = marker_bit(m);
    let _ = cfg.update_command(|v| (v & !bit) | (s.command & bit));
}

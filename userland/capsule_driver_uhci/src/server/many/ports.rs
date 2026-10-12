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
//! The host's port list: each controller's own, its ports numbered on
//! after the controllers before it, then the virtual ports of the
//! children parked below hubs with their extensions (nonos_usb_virt's
//! report.rs has the bytes).

use super::front::{Bind, Front};
use super::hc::Hc;
use crate::server::reply::{data, DATA_AT};
use crate::wire::Request;
use nonos_usb_virt::{ext, port_entry};

/// One controller's list fits here: 15 ports of 8 bytes past its head.
const ONE: usize = DATA_AT + 4 + 8 * 15;

pub fn ports<H: Hc>(f: &mut Front<H>, req: &Request, tx: &mut [u8]) -> usize {
    let mut total = 0usize;
    for k in 0..f.hcs.len() {
        let base = f.base(k);
        let Some(hc) = f.hcs[k].as_mut() else { continue };
        let mut one = [0u8; ONE];
        hc.serve(req, &[], &mut one);
        let n = (one[DATA_AT] as usize).min(15);
        for e in one[DATA_AT + 4..DATA_AT + 4 + 8 * n].chunks_exact(8) {
            let at = DATA_AT + 4 + 8 * total;
            tx[at..at + 8].copy_from_slice(e);
            tx[at] = base.saturating_add(e[0]);
            total += 1;
        }
    }
    let roots = total;
    for (vport, p) in f.virt.entries() {
        let held = match f.bound.get(p.slot as usize - 1) {
            Some(Bind::On(k)) => f.hcs[*k].as_ref().map_or(1, |h| h.held(p.slot)),
            _ => 1,
        };
        let at = DATA_AT + 4 + 8 * total;
        tx[at..at + 8].copy_from_slice(&port_entry(vport, p, held));
        total += 1;
    }
    let mut end = DATA_AT + 4 + 8 * total;
    for (vport, p) in f.virt.entries() {
        tx[end..end + 8].copy_from_slice(&ext(vport, p));
        end += 8;
    }
    f.virt.reported();
    tx[DATA_AT..DATA_AT + 4].copy_from_slice(&[total as u8, (total - roots) as u8, 0, 0]);
    data(tx, req, end - DATA_AT)
}

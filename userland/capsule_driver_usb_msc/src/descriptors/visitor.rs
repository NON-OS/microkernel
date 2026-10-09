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

use super::types::{MscBinding, ProbeResult};
use super::wire::*;
use crate::protocol::MAX_BINDINGS;

/// `last_ep` is the address of the bulk endpoint just bound, which a
/// SuperSpeed companion right after it describes; 0 when there is none.
pub(super) fn visit_record(
    rec: &[u8],
    cur: &mut Option<MscBinding>,
    last_ep: &mut u8,
    out: &mut ProbeResult,
) {
    match rec[1] {
        DESC_INTERFACE => {
            *last_ep = 0;
            visit_interface(rec, cur)
        }
        DESC_ENDPOINT => *last_ep = visit_endpoint(rec, cur, out),
        DESC_SS_EP_COMPANION => {
            visit_companion(rec, *last_ep, cur, out);
            *last_ep = 0;
        }
        _ => {}
    }
}

fn visit_interface(rec: &[u8], cur: &mut Option<MscBinding>) {
    if rec.len() < 9 {
        *cur = None;
        return;
    }
    let is_msc = rec[5] == CLASS_MASS_STORAGE
        && rec[6] == SUBCLASS_SCSI_TRANSPARENT
        && rec[7] == PROTOCOL_BULK_ONLY;
    *cur = is_msc.then_some(MscBinding { interface: rec[2], ..MscBinding::default() });
}

/// Bind a bulk endpoint of the current interface; returns its address, or 0
/// when the record bound nothing.
fn visit_endpoint(rec: &[u8], cur: &mut Option<MscBinding>, out: &mut ProbeResult) -> u8 {
    let Some(mut binding) = *cur else { return 0 };
    if rec.len() < 7 || rec[3] & EP_ATTR_TRANSFER_MASK != EP_ATTR_BULK {
        return 0;
    }
    bind_endpoint(rec, &mut binding);
    *cur = Some(binding);
    if binding.bulk_in != 0 && binding.bulk_out != 0 && out.count < MAX_BINDINGS {
        out.bindings[out.count] = binding;
        out.count += 1;
        *cur = None;
    }
    rec[2]
}

/// A SuperSpeed companion: its bMaxBurst (bits 4:0, 0 to 15 allowed) goes
/// to the endpoint it follows, in the interface being walked or the binding
/// that endpoint just completed. A value past 15 is out of specification
/// and taken as 0, one packet per burst.
fn visit_companion(rec: &[u8], ep: u8, cur: &mut Option<MscBinding>, out: &mut ProbeResult) {
    if rec.len() < 6 || ep == 0 {
        return;
    }
    let burst = if rec[2] <= 15 { rec[2] } else { 0 };
    let last = out.count.checked_sub(1);
    let binding = match cur {
        Some(b) if b.bulk_in == ep || b.bulk_out == ep => b,
        _ => match last.map(|i| &mut out.bindings[i]) {
            Some(b) if b.bulk_in == ep || b.bulk_out == ep => b,
            _ => return,
        },
    };
    if ep & EP_DIR_IN != 0 {
        binding.max_burst_in = burst;
    } else {
        binding.max_burst_out = burst;
    }
}

fn bind_endpoint(rec: &[u8], binding: &mut MscBinding) {
    let max_packet = u16::from_le_bytes([rec[4], rec[5]]);
    if rec[2] & EP_DIR_IN != 0 {
        binding.bulk_in = rec[2];
        binding.max_packet_in = max_packet;
    } else {
        binding.bulk_out = rec[2];
        binding.max_packet_out = max_packet;
    }
}

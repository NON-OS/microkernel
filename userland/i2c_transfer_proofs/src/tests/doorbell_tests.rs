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

//! OP_GPIO_DOORBELL senses the bound declaration's interrupt line. Once the
//! client has found another pad and the reach has narrowed to it, that line
//! belongs to a pad the client does not read, so the doorbell answers that
//! there is none, and the HID driver polls instead of waiting on it. The
//! pad setup's probe bound, which is most machines, keeps its line.

use nonos_libc::take_reply;

use super::client::transfer_errno;
use super::kept::{reply_buffer, wiped};
use super::multi_sku::{bound, FITTED, FITTED_REG};
use crate::driver::{Doorbell, Driver};
use crate::protocol::{parse, Request, E_OK, OP_GPIO_DOORBELL};
use crate::regs::{Attached, Regs};
use crate::server::handlers::doorbell;
use crate::transaction::Presence;

/// (present, fired), as the doorbell answers them.
fn rung(d: &Driver) -> (u32, u32) {
    let mut out = reply_buffer();
    doorbell::handle(d, 42, &Request { op: OP_GPIO_DOORBELL, request_id: 3 }, &mut out);
    let (_, bytes) = take_reply().expect("the handler replied");
    assert!(wiped(&out), "the reply buffer kept the answer");
    let (_, p) = parse(&bytes).expect("the reply parses");
    assert_eq!((p.len(), &p[..4]), (12, &E_OK.to_le_bytes()[..]));
    let word = |at: usize| u32::from_le_bytes([p[at], p[at + 1], p[at + 2], p[at + 3]]);
    (word(4), word(8))
}

/// Bound as setup binds once its probe `found` what it says, with the bound
/// declaration's line an active-low one, read from a register that holds
/// zero: asserted.
fn with_line(found: Presence) -> (Driver, Attached) {
    let (mut d, a) = bound(found);
    let regs = Regs::new(0, 0x1000);
    d.doorbell = Some(Doorbell { regs, cfg_offset: 0x0F00, level_bit: 1 << 1, active_high: false });
    (d, a)
}

#[test]
fn the_bound_pads_line_stops_gating_reads_once_the_client_found_another_pad() {
    let (d, _a) = with_line(Presence::Absent);
    assert_eq!(rung(&d), (1, 1), "the bound pad's line, while that pad is in reach");
    assert_eq!(transfer_errno(&d, FITTED, &FITTED_REG.to_le_bytes(), 30), E_OK);
    assert_eq!(rung(&d), (0, 0), "the line of a pad out of reach still gates reads");
}

#[test]
fn a_pad_setups_probe_bound_keeps_its_line_before_and_after_the_client_reads_it() {
    let (d, _a) = with_line(Presence::HidDescriptor(FITTED_REG));
    assert_eq!(rung(&d), (1, 1), "the probe-bound pad's line went silent");
    assert_eq!(transfer_errno(&d, FITTED, &FITTED_REG.to_le_bytes(), 30), E_OK);
    assert_eq!(rung(&d), (1, 1), "the line went silent once the client read its pad");
}

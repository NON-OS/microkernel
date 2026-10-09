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

//! The PCIe Advanced Error Reporting registers the boot report reads, and the
//! names of their status bits. Pure, so the bit positions are held against the
//! spec on the host (kernel_proofs aer_decode).
//!
//! PCIe Base 5.0, 7.8.4: the AER extended capability has id 0001h; the
//! Uncorrectable Error Status register sits at +04h and the Correctable Error
//! Status register at +10h. Names are the spec's in lower case; bit 20 is
//! unsup-req, shortened as Linux aer.c names it (UnsupReq).

pub(crate) const AER_CAP_ID: u16 = 0x0001;
pub(crate) const UNCOR_STATUS: u16 = 0x04;
pub(crate) const COR_STATUS: u16 = 0x10;

/// PCIe Base 5.0, 7.8.4.2.
pub(crate) const UNCORRECTABLE: &[(u32, &str)] = &[
    (4, "data-link-protocol"),
    (5, "surprise-down"),
    (12, "poisoned-tlp"),
    (13, "flow-control-protocol"),
    (14, "completion-timeout"),
    (15, "completer-abort"),
    (16, "unexpected-completion"),
    (17, "receiver-overflow"),
    (18, "malformed-tlp"),
    (19, "ecrc"),
    (20, "unsup-req"),
    (21, "acs-violation"),
    (22, "uncorrectable-internal"),
    (23, "mc-blocked-tlp"),
    (24, "atomicop-egress-blocked"),
    (25, "tlp-prefix-blocked"),
    (26, "poisoned-tlp-egress-blocked"),
];

/// PCIe Base 5.0, 7.8.4.5.
pub(crate) const CORRECTABLE: &[(u32, &str)] = &[
    (0, "receiver-error"),
    (6, "bad-tlp"),
    (7, "bad-dllp"),
    (8, "replay-num-rollover"),
    (12, "replay-timer-timeout"),
    (13, "advisory-non-fatal"),
    (14, "corrected-internal"),
    (15, "header-log-overflow"),
];

/// The names of the set bits a table knows, lowest bit first.
pub(crate) fn names(
    status: u32,
    table: &'static [(u32, &'static str)],
) -> impl Iterator<Item = &'static str> {
    table.iter().filter(move |(bit, _)| status & (1 << bit) != 0).map(|(_, name)| *name)
}

/// An extended capability header's id and the offset of the next one
/// (PCIe Base 5.0, 7.6.3). An offset under 100h ends the list.
pub(crate) const fn ext_header(header: u32) -> (u16, u16) {
    ((header & 0xFFFF) as u16, ((header >> 20) & 0xFFC) as u16)
}

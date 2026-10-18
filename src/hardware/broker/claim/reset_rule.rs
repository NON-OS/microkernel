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

//! A function's reset before its frames are freed or it reaches a new
//! claimant, pure. With Bus Master already clear: wait, bounded,
//! for Transactions Pending to clear; save the header; Function Level
//! Reset; wait the 100 ms PCIe 5.0 6.6.2 asks before a config request;
//! wait, bounded, for the function to answer; restore the header. A
//! function that offers no FLR is left as it is.
//! kernel_proofs::function_reset.

/// Transactions Pending may stay set this long; then the reset goes ahead
/// anyway, as Linux's pci_wait_for_pending_transaction does after 700 ms.
pub(crate) const PENDING_MS: u64 = 700;
/// No config request this soon after the FLR (PCIe 5.0, 6.6.2).
pub(crate) const SETTLE_MS: u64 = 100;
/// After that, how long the function may still not answer.
pub(crate) const READY_MS: u64 = 1_000;
pub(crate) const POLL_MS: u64 = 10;

/// The FLR a function offers: through its PCIe capability, or through the
/// Advanced Features capability of a conventional PCI function.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Flr {
    Pcie,
    Af,
}

/// Device Capabilities bit 28, Function Level Reset Capability.
pub(crate) const fn pcie_flr(devcap: u32) -> bool {
    devcap & (1 << 28) != 0
}

/// AF Capabilities: TP (bit 0) and FLR (bit 1), both, as Linux asks.
pub(crate) const fn af_flr(afcap: u8) -> bool {
    afcap & 0b11 == 0b11
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Reset {
    NotOffered,
    /// Reset and restored; `pending_left` when Transactions Pending
    /// outlasted PENDING_MS.
    Done {
        pending_left: bool,
    },
    /// Reset, and silent for READY_MS after: its header is not restored.
    NotReady,
}

pub(crate) trait ResetIo {
    fn flr(&mut self) -> Option<Flr>;
    fn pending(&mut self, how: Flr) -> bool;
    fn save(&mut self);
    fn initiate(&mut self, how: Flr);
    fn ready(&mut self) -> bool;
    fn restore(&mut self);
    fn now_ms(&mut self) -> u64;
    fn pause_ms(&mut self, ms: u64);
}

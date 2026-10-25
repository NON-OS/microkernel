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

//! Who may ask for which job (plan/15), decided from facts the handler
//! looked up, so the rule is tested on its own. Kind 1 PROVE: the caller
//! owns app.store. Kind 2 DELIVER: the caller is at Vendor authority and
//! supervises a live family. Anything else is no ask at all.

pub(crate) const PROVE: u64 = 1;
pub(crate) const DELIVER: u64 = 2;

/// The store's pid; or the supervisor's pid and its measurement.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ProveAsk {
    Prove(u32),
    Deliver(u32, [u8; 32]),
}

/// What the kernel knows of the caller when it asks.
pub(crate) struct Caller {
    pub(crate) pid: u32,
    pub(crate) owns_store: bool,
    /// Its measurement, when the spawn gate proved it at Vendor authority.
    pub(crate) vendor: Option<[u8; 32]>,
    pub(crate) supervises_live: bool,
}

pub(crate) fn ask_for(kind: u64, c: &Caller) -> Option<ProveAsk> {
    match kind {
        PROVE if c.owns_store => Some(ProveAsk::Prove(c.pid)),
        DELIVER if c.supervises_live => c.vendor.map(|m| ProveAsk::Deliver(c.pid, m)),
        _ => None,
    }
}

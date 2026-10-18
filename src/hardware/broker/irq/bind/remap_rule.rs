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

//! Which message a device's interrupt gets (K3), pure. Under remapping only
//! an entry the kernel wrote for the device's own requester id and vector;
//! a compatibility format message, which can name any vector or NMI, INIT
//! or SMI on any CPU, only while remapping is off and under the
//! development opt-in. kernel_proofs::iommu_remap::rule_tests.

/// How asking the remapping table for an entry ended. Built without
/// remapping, every attempt is Off.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(not(feature = "nonos-iommu-intremap"), allow(dead_code))]
pub(crate) enum Attempt {
    /// An entry was written for the device.
    Routed,
    /// Remapping is on and its table has no free entry.
    Full,
    /// Remapping is on and this device cannot have an entry: another PCI
    /// segment, or a destination the entry format cannot name.
    Failed,
    /// Remapping is off, or not built.
    Off,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Outcome {
    Remapped,
    /// A compatibility format message, under the opt-in alone.
    Compat,
    Full,
    Refused,
}

pub(crate) const fn outcome(attempt: Attempt, opted_in: bool) -> Outcome {
    match attempt {
        Attempt::Routed => Outcome::Remapped,
        Attempt::Full => Outcome::Full,
        Attempt::Failed => Outcome::Refused,
        Attempt::Off if opted_in => Outcome::Compat,
        Attempt::Off => Outcome::Refused,
    }
}

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

//! What a PCI root bridge's _OSC left to the operating system: the controls
//! firmware offered when asked for all of them (its query answer), and the
//! ones NONOS took (its commit answer), with the buses the bridge decodes.
//! A's AER, and hot plug when it comes, read this before touching what
//! firmware may still own. The control bits are the PCI Firmware
//! specification's _OSC control field, recalled rather than read:
//! unverified. Pure: `kernel_proofs` mounts it.

pub const NATIVE_HOT_PLUG: u32 = 1 << 0;
pub const SHPC_HOT_PLUG: u32 = 1 << 1;
pub const PME: u32 = 1 << 2;
pub const AER: u32 = 1 << 3;
pub const PCIE_CAPABILITY: u32 = 1 << 4;
pub const LTR: u32 = 1 << 5;
pub const DPC: u32 = 1 << 7;
/// Everything the query asks for.
pub const ALL: u32 = NATIVE_HOT_PLUG | SHPC_HOT_PLUG | PME | AER | PCIE_CAPABILITY | LTR | DPC;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OscGrant {
    pub segment: u16,
    pub first: u8,
    pub last: u8,
    /// Would firmware give it up.
    pub offered: u32,
    /// Did NONOS take it; firmware still owns everything else.
    pub granted: u32,
}

impl OscGrant {
    /// Firmware gave up `control`, and NONOS took it.
    pub fn os_owns(&self, control: u32) -> bool {
        control != 0 && self.granted & control == control
    }
}

/// None for a record that claims buses backwards or a grant past the
/// offer, which no honest commit answers.
pub fn checked(g: OscGrant) -> Option<OscGrant> {
    let sound = g.first <= g.last && g.granted & !g.offered == 0;
    sound.then_some(g)
}

/// The controls the kernel would ever run natively: none while native hot
/// plug is off (the capsule's `NATIVE_HOT_PLUG_ON`; the two move together).
/// The kernel's own, so it records no grant wider whatever it is sent.
pub const KERNEL_CLAIM: u32 = 0;

/// The grant cut to `claim`, and the bits cut, for the caller to say once.
pub fn within_claim(g: OscGrant, claim: u32) -> (OscGrant, u32) {
    (OscGrant { granted: g.granted & claim, ..g }, g.granted & !claim)
}

/// The grant of the root bridge whose buses hold `bus`.
pub fn grant_for(grants: &[OscGrant], segment: u16, bus: u8) -> Option<OscGrant> {
    let holds = |g: &&OscGrant| g.segment == segment && (g.first..=g.last).contains(&bus);
    grants.iter().find(holds).copied()
}

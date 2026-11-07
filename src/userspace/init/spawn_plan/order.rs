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

//! The driver stages init spawns, each with what it needs up first, and the
//! order that follows from them: a topological order of the declared graph,
//! ties broken by declaration order, so it is the same on every boot and
//! moves only when a declaration does. Pure: `kernel_proofs` mounts it.
//!
//! What a stage needs, and why:
//! - `PlatformAcpi` is the power service and the ACPI capsule, and init's
//!   wait for the capsule's first publish pass. By then ACPI mode is on, the
//!   EC's region is connected and its _REG run, every _INI has run, and the
//!   hosts and power resources only ACPI names are published (SD, eMMC,
//!   GPIO and I2C hosts, functions woken by _PS0).
//! - Disks, USB hosts, the platform buses, input, NICs and audio find hosts
//!   ACPI publishes, or functions it powers: after `PlatformAcpi`.
//! - The platform buses (GPIO, I2C, SMBus, SPI, Wi-Fi) after the USB hosts:
//!   a CNVi Wi-Fi card's isolation group with xHCI goes to USB first.
//! - Input after the buses: the touchpad's I2C host and its GPIO interrupt.
//! - USB class drivers after their hosts; USB networking after them.
//! - The Hyper-V devices after the VMBus.

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Stage {
    Virtio,
    PlatformAcpi,
    VmBus,
    VmDisplay,
    Disks,
    UsbHosts,
    Buses,
    Input,
    VmInput,
    Nics,
    VmNics,
    UsbClass,
    UsbNet,
    Storage,
    VmStorage,
    VmAudio,
    Audio,
}

use Stage::*;

/// Each stage and what must be up before it, in declaration order.
pub(super) const GRAPH: [(Stage, &[Stage]); 17] = [
    (Virtio, &[]),
    (PlatformAcpi, &[]),
    (VmBus, &[]),
    (VmDisplay, &[VmBus]),
    (Disks, &[PlatformAcpi]),
    (UsbHosts, &[PlatformAcpi]),
    (Buses, &[PlatformAcpi, UsbHosts]),
    (Input, &[PlatformAcpi, Buses]),
    (VmInput, &[VmBus]),
    (Nics, &[PlatformAcpi]),
    (VmNics, &[VmBus]),
    (UsbClass, &[UsbHosts]),
    (UsbNet, &[UsbClass]),
    (Storage, &[Disks, UsbClass]),
    (VmStorage, &[VmBus]),
    (VmAudio, &[VmBus]),
    (Audio, &[PlatformAcpi, UsbClass]),
];

pub(super) const STAGES: usize = GRAPH.len();

/// The stages in order: at each step the first declared stage whose needs
/// are all placed. None if the graph has a cycle or names a stage it does
/// not declare.
pub(super) const fn order() -> Option<[Stage; STAGES]> {
    let mut out = [Virtio; STAGES];
    let mut placed = [false; STAGES];
    let mut n = 0;
    while n < STAGES {
        let mut pick = STAGES;
        let mut i = 0;
        while i < STAGES && pick == STAGES {
            if !placed[i] && ready(i, &placed) {
                pick = i;
            }
            i += 1;
        }
        if pick == STAGES {
            return None;
        }
        placed[pick] = true;
        out[n] = GRAPH[pick].0;
        n += 1;
    }
    Some(out)
}

const fn index(s: Stage) -> usize {
    let mut i = 0;
    while i < STAGES {
        if GRAPH[i].0 as u8 == s as u8 {
            return i;
        }
        i += 1;
    }
    STAGES
}

const fn ready(i: usize, placed: &[bool; STAGES]) -> bool {
    let needs = GRAPH[i].1;
    let mut k = 0;
    while k < needs.len() {
        let at = index(needs[k]);
        if at == STAGES || !placed[at] {
            return false;
        }
        k += 1;
    }
    true
}

/// Whether `s` needs the ACPI stage, directly or through a stage it needs:
/// these are walked again when the capsule publishes late (`late`).
pub(super) const fn after_acpi(s: Stage) -> bool {
    reaches(s, PlatformAcpi, STAGES)
}

/// Whether `s` needs `target` within `depth` steps; the graph has no cycle,
/// so `STAGES` steps reach every need.
const fn reaches(s: Stage, target: Stage, depth: usize) -> bool {
    let at = index(s);
    if depth == 0 || at == STAGES {
        return false;
    }
    let needs = GRAPH[at].1;
    let mut k = 0;
    while k < needs.len() {
        if needs[k] as u8 == target as u8 || reaches(needs[k], target, depth - 1) {
            return true;
        }
        k += 1;
    }
    false
}

const DECLARED: [Stage; STAGES] = {
    let mut out = [Virtio; STAGES];
    let mut i = 0;
    while i < STAGES {
        out[i] = GRAPH[i].0;
        i += 1;
    }
    out
};

/// Computed when the kernel is built; a graph with a cycle does not build.
pub(super) const ORDER: [Stage; STAGES] = match order() {
    Some(o) => o,
    None => DECLARED,
};

const _: () = assert!(order().is_some(), "the driver stages' graph has a cycle");

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

/* The vendor a guest kernel can tell apart, and the slug its boot line uses. */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hypervisor {
    None = 0,
    Kvm = 1,
    QemuTcg = 2,
    Vmware = 3,
    HyperV = 4,
    VirtualBox = 5,
    Xen = 6,
    Bhyve = 7,
    Acrn = 8,
    Parallels = 9,
    Unknown = 10,
}

impl Hypervisor {
    pub const fn slug(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Kvm => "kvm",
            Self::QemuTcg => "qemu-tcg",
            Self::Vmware => "vmware",
            Self::HyperV => "hyperv",
            Self::VirtualBox => "virtualbox",
            Self::Xen => "xen",
            Self::Bhyve => "bhyve",
            Self::Acrn => "acrn",
            Self::Parallels => "parallels",
            Self::Unknown => "unknown",
        }
    }

    // Anything outside the table reads as unknown rather than as bare metal:
    // a corrupted memo must never switch a guest's paravirt paths off silently
    // into the paths that assume real hardware.
    pub const fn from_code(code: u8) -> Self {
        match code {
            0 => Self::None,
            1 => Self::Kvm,
            2 => Self::QemuTcg,
            3 => Self::Vmware,
            4 => Self::HyperV,
            5 => Self::VirtualBox,
            6 => Self::Xen,
            7 => Self::Bhyve,
            8 => Self::Acrn,
            9 => Self::Parallels,
            _ => Self::Unknown,
        }
    }
}

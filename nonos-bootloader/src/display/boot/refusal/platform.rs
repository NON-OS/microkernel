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

//! Refusals about the machine and the rollback floor.

use super::advice::Advice;

pub const PLATFORM: [(&[u8], Advice); 4] = [
    (b"Rollback", Advice {
        title: b"This kernel is older than allowed",
        what: b"Its rollback index is below the floor this machine keeps, so a newer release already booted here. Older releases stay refused so known flaws cannot come back.",
        remedy: b"Boot a release at least as new as the last one used on this machine.",
    }),
    (b"Security policy enforcement failed", Advice {
        title: b"This machine lacks what the entry requires",
        what: b"The entry you chose needs hardware or firmware settings this machine does not have on. The boot menu lists what is missing under each entry.",
        remedy: b"Restart and choose an entry the menu shows as met here, or turn on Secure Boot, the TPM and RDRAND in the firmware.",
    }),
    (b"Insufficient entropy", Advice {
        title: b"Not enough randomness",
        what: b"The loader could not gather enough randomness from the hardware to make this boot's keys.",
        remedy: b"Turn on RDRAND or the TPM in the firmware settings. A virtual machine needs a virtio-rng device.",
    }),
    (b"Hardware requirements not met", Advice {
        title: b"This machine is not supported",
        what: b"The processor lacks a feature N\xD8NOS needs, such as NX or 36-bit physical addresses.",
        remedy: b"Use a 64-bit machine from the last ten years, or check the firmware settings.",
    }),
];

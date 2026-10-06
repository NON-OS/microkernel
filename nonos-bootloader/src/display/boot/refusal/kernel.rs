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

//! Refusals about the kernel image: finding it, its signatures, its STARK
//! attestation, its rollback index and its ELF.

use super::advice::Advice;

pub const KERNEL: [(&[u8], Advice); 6] = [
    (b"Kernel not found", Advice {
        title: b"No kernel on this disk",
        what: b"The loader looked for EFI/nonos/kernel.bin on the EFI partition and found no kernel.",
        remedy: b"Boot from the N\xD8NOS media you made, or write the release image to it again.",
    }),
    (b"Kernel not signed", Advice {
        title: b"The kernel is not signed",
        what: b"The kernel carries no release signature, so the loader cannot tell who built it.",
        remedy: b"Boot an official N\xD8NOS release, or sign your own build with sign-kernel.",
    }),
    (b"Signature invalid", Advice {
        title: b"The kernel signature does not match",
        what: b"Its Ed25519 and ML-DSA-65 signatures did not both verify against the keys built into this loader. The kernel was changed, or signed with another key.",
        remedy: b"Write the boot media again from a release you trust. Loader and kernel must come from the same release.",
    }),
    (b"Kernel self-attestation invalid", Advice {
        title: b"The kernel's STARK attestation failed",
        what: b"Its STARK proof or its path did not verify against the root enrolled in this loader. The kernel is not the one this release enrolled.",
        remedy: b"Use the loader and kernel of one release together, and write the boot media again.",
    }),
    (b"kernel attestation required", Advice {
        title: b"The kernel has no STARK attestation",
        what: b"This loader boots only a kernel whose measurement is enrolled, with a STARK proof. This kernel carries none, or one for another kind.",
        remedy: b"Use the loader and kernel of one release together, and write the boot media again.",
    }),
    (b"Kernel ELF parsing failed", Advice {
        title: b"The kernel could not be read",
        what: b"The kernel passed its checks but its program headers could not be loaded.",
        remedy: b"The media may be damaged. Write it again; if it repeats, report this screen.",
    }),
];

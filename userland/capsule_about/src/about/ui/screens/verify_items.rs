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

//! The claims this window cannot settle, and where each is settled instead.
//!
//! The second line of each is not an excuse; it is the pointer a sceptical
//! reader needs in order to go and check for themselves.

/// Claim, then where it is settled instead.
pub(super) const ITEMS: [(&[u8], &[u8]); 3] = [
    (
        b"every running capsule was signature-checked",
        b"settled by the kernel at spawn, which admits no unverified image. Re-checking it needs the trust store on disk, and this window holds no FileSystem capability, which is why it cannot.",
    ),
    (
        b"nothing a capsule held survives its exit",
        b"settled by kernel teardown, which zeroizes the address space. A peer capsule cannot observe another's teardown and must not be able to.",
    ),
    (
        b"the syscall surface is not POSIX",
        b"settled at build time by the syscall table itself. It is a property of the image, not a state that could change while it runs.",
    ),
];

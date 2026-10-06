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

//! A slot of a policy tree, and the context its gate builds for it.

use nonos_attest_path::Kind;

use crate::context::{capsule_context, kernel_context};

/// What a slot holds: the image's BLAKE3 and, for a capsule, its capability word.
#[derive(Clone, Copy)]
pub enum Slot {
    Kernel([u8; 32]),
    Capsule([u8; 32], u64),
    /// Same context shape as a kernel (measurement, boot epoch); the kind is
    /// what keeps the two apart.
    Bootloader([u8; 32]),
}

impl Slot {
    pub fn context(&self) -> (Kind, Vec<u8>) {
        match self {
            Slot::Kernel(h) => (Kind::Kernel, kernel_context(h)),
            Slot::Bootloader(h) => (Kind::Bootloader, kernel_context(h)),
            Slot::Capsule(h, caps) => (Kind::Capsule, capsule_context(h, *caps)),
        }
    }
}

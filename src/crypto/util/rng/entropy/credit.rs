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

//! What the entropy capsule's bytes count for, pure (K9): they go into a
//! seed with the CPU's sources and count toward it for at most MAX_CREDIT
//! bytes, so they are never enough alone. kernel_proofs::entropy_health.

pub(crate) const SEED_BYTES: usize = 32;
pub(crate) const MAX_CREDIT: usize = SEED_BYTES / 2;
const _: () = assert!(MAX_CREDIT < SEED_BYTES);

/// Whether `cpu` bytes from the CPU and `capsule` held make a seed.
pub(crate) fn enough(cpu: usize, capsule: usize) -> bool {
    cpu + capsule.min(MAX_CREDIT) >= SEED_BYTES
}

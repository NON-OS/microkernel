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

/* Mirrors the kernel `crate::memory::phys` parent for the included span walk. */
#[path = "../../../../../src/memory/phys/constants/mod.rs"]
pub mod constants;

#[path = "../../../../../src/memory/phys/usable_span.rs"]
pub mod usable_span;

/// The kernel's walk of the usable stretches of a span, for the proofs: the
/// gaps it hands to `gap` and the bytes of whole frames the regions cover.
pub fn walk_usable(sorted: &[(u64, u64)], start: u64, end: u64, gap: impl FnMut(u64, u64)) -> u64 {
    usable_span::walk_usable(sorted, start, end, gap)
}

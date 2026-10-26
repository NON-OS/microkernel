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

//! The kernel's placement (`avoid::place_avoiding`) over held regions alone,
//! as the reserved tests place: it steps over each as over the interrupt
//! window, so no grant's IOVA lands on a region mapped one to one. The kernel
//! passes these and the reserved ranges together (`reserved_spans`).

use super::avoid::place_avoiding;
use super::reserved_rule::Held;

pub(super) fn place_clear(next: u64, length: u64, held: &[Held]) -> Option<(u64, u64)> {
    let spans: Vec<(u64, u64)> = held.iter().map(|h| (h.base, h.end)).collect();
    place_avoiding(next, length, &spans)
}

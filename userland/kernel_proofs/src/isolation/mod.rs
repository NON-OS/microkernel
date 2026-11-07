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
//! Cross-capsule isolation, as a standing regression: capsule A never maps
//! or reads a frame of capsule B's unless B named A while the surface lived,
//! and never after B took it back, released it, or exited, or after A's own
//! pid went to a new process. Every decision on the way is the kernel's own
//! code by `#[path]`: the slot's named receivers (`share/rule.rs`), the
//! rights with their write bit (`share/rights_rule.rs`, `rights_attach.rs`),
//! the exiting owner's closing list (`pin/closing_rule.rs`), and where the
//! view lands (`process/mmap_va.rs`), with handles encoded as the kernel encodes
//! them (`handle.rs`). The slot table and the order of the
//! checks are restated from `share/attach_surface.rs` and `permit.rs`.
//!
//! What this does not run: the page tables and the TLB. That a view
//! mapped into A's ASID is reachable from no other ASID is the paging
//! code's, and is checked on hardware by capsule_attack's isolation lines.

#[path = "../../../../src/kernel_core/surface_registry/share/rule.rs"]
mod rule;
#[path = "../../../../src/kernel_core/surface_registry/share/rights_attach.rs"]
mod rights_attach;
#[path = "../../../../src/kernel_core/surface_registry/share/rights_rule.rs"]
mod rights_rule;
#[path = "../../../../src/kernel_core/surface_registry/pin/closing_rule.rs"]
mod closing_rule;
#[path = "../../../../src/kernel_core/surface_registry/handle.rs"]
mod handle;

mod model;
mod tests;

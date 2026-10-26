// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Which way a kernel image is loaded, and how many bytes it needs. Pure, so
//! the rules are held on the host (loader_arm_proofs).

/// How `load_kernel` loads an image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// An ET_EXEC where its ELF says, or anywhere mapped high (x86_64).
    Exec,
    /// An ET_DYN the loader relocates (x86_64).
    Dyn,
    /// A self-relocating ET_DYN in an aligned block at its link offset.
    Block,
    Refuse(&'static str),
}

/// `relocating` is an architecture whose kernel relocates itself and is
/// entered with the MMU off (aarch64): only a low-linked ET_DYN can be
/// placed and run there, so anything else is refused, not moved blind.
pub fn route(is_exec: bool, is_dyn: bool, upper_half: bool, relocating: bool) -> Route {
    match (relocating, is_exec, is_dyn) {
        (true, _, true) if !upper_half => Route::Block,
        (true, _, _) => Route::Refuse("this CPU takes a self-relocating ET_DYN kernel linked low"),
        (false, true, _) => Route::Exec,
        (false, false, true) => Route::Dyn,
        (false, false, false) => Route::Refuse("unsupported ELF type"),
    }
}

/// The bytes to place: the segments' `span`, raised to what the image's
/// header `claimed`. None when the header is `required` and missing, since
/// then the boot stack past the last segment would lie in memory nobody
/// allocated.
pub fn footprint_rule(claimed: Option<u64>, span: usize, required: bool) -> Option<usize> {
    match claimed {
        Some(size) => Some(span.max(usize::try_from(size).ok()?)),
        None if required => None,
        None => Some(span),
    }
}

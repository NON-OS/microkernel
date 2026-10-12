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

//! The attributes kernel text is mapped with: read only, executable at EL1,
//! and guarded where BTI will check every indirect branch into it. The one
//! place the tables learn whether that holds, so the host proofs can stand
//! in for it.

use super::PageAttributes;

pub(super) fn code() -> PageAttributes {
    PageAttributes::kernel_code().with_guard(crate::arch::aarch64::security::kernel_guarded())
}

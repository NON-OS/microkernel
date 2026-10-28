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

//! Preparing this computer for an update, from the installer on the
//! installed system: the kernel hands the data volume over, so that the
//! next release, booted from its stick, can open it once and keep it. What
//! the handover is and why it is safe is the kernel's to say
//! (`docs/security/measured-boot-and-tpm.md`); this asks for it and says
//! what came back.

mod answer;
mod call;

pub use answer::answer;

/// Ask the kernel to prepare, and keep its answer for the screen.
pub fn prepare() -> i64 {
    call::mk_data_handover()
}

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

/*
 * A NOP is not a BTI landing pad, and PACIASP and PACIBSP are.
 *
 * The kernel's landing-pad check and its decoder are included by path. The
 * check accepted the NOP, which faults as a branch target on a core with
 * FEAT_BTI, and rejected PACIASP and PACIBSP, which act as BTI c. The check
 * below fails against that code.
 */

#[path = "../../../../src/arch/aarch64/security/bti/landing.rs"]
pub mod landing;
#[path = "../../../../src/arch/aarch64/security/bti/pad.rs"]
pub mod pad;
mod tests;

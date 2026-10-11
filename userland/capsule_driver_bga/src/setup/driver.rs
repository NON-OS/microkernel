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

use super::surface::Shadow;
use crate::dispi::Mode;
use crate::gfx::Screen;
use crate::handles::Claim;

/// Everything a served adapter holds. Field order is drop order: the shadow
/// is released before the claim that took the VRAM mapping goes. The claim
/// is held only for that drop, which releases the adapter.
pub struct Driver {
    pub shadow: Shadow,
    pub _claim: Claim,
    pub fb_va: u64,
    pub mode: Mode,
    pub screen: Screen,
}

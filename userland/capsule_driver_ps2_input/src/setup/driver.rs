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
pub struct Driver {
    pub pio_grant_id: u64,
    pub irq_grant_id: u64,
    pub aux_irq_grant_id: u64,
    pub mouse_enabled: bool,
    /// The mouse negotiated the 4-byte IntelliMouse report with a scroll wheel.
    pub mouse_wheel: bool,
}

impl Driver {
    /// Whether a port in use has no line bound (a grant id of 0): its bytes
    /// are found only by draining the controller, so the serve loop must not
    /// sleep long between passes. A mouse packet is three or four bytes, and
    /// the controller holds one at a time.
    pub fn polled(&self) -> bool {
        self.irq_grant_id == 0 || (self.mouse_enabled && self.aux_irq_grant_id == 0)
    }
}

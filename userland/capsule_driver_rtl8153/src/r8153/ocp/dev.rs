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

//! The chip behind its control pipe: the bus, and which PHY page the
//! PLA_OCP_GPHY_BASE window shows now (Linux tp->ocp_base).

pub struct Dev<B> {
    pub bus: B,
    /// `None` until the first PHY access sets the window, as Linux starts
    /// from an ocp_base of -1 that no page matches.
    pub(super) ocp_base: Option<u16>,
}

impl<B> Dev<B> {
    pub fn new(bus: B) -> Self {
        Self { bus, ocp_base: None }
    }
}

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

//! The flag this driver and the manageability firmware share before either
//! touches the PHY or resets the MAC: EXTCNF_CTRL bit 5, SWFLAG on the PCH
//! parts, MDIO_SW_OWNERSHIP on the 82574 and 82583.

mod acquire;
mod release;

pub use acquire::acquire;
pub use release::release;

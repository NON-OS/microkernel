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

//! The internal copper PHY at MII address 1, reached through MAC_MI_COM, and
//! tg3's way of bringing its link up and reading what it negotiated.

pub mod aux;
pub mod copper;
mod established;
mod mac_side;
pub mod mdio;
pub mod negotiate;
pub mod pause;
pub mod reset;
pub mod resolve;

pub use established::link_bit;
pub use mdio::Mdio;

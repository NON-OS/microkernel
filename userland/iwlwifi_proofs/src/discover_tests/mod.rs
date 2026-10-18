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

//! Discovery finds an Intel Wi-Fi adapter whatever firmware left in its INTx
//! registers, and finds nothing else; a refused interrupt bind ends the
//! attempt only when the claim is gone. Laptops whose firmware leaves the
//! legacy line at 0xFF (the OS is expected to use MSI/MSI-X) used to have
//! their adapter skipped at discovery.

mod find;
mod ladder;

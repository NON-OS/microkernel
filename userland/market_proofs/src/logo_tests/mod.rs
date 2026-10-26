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

//! OP_GET_LOGO's check and wire, included from the market capsule: a logo
//! is served only when its bytes hash to the logo_hash the signed index
//! carries, and the store reads back exactly what was served.

#[path = "../../../capsule_market/src/logos/answer.rs"]
mod answer;
#[path = "../../../capsule_market/src/logos/table.rs"]
mod table;

mod serve;
mod wire;

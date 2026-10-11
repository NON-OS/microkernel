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

//! The ITS's register and command formats (GICv3 and GICv4 architecture
//! specification, IHI 0069, chapters 5 and 12). Pure, so every field is
//! proved on the host against the specification (gic_proofs).

pub mod attrs;
pub mod baser;
pub mod cmd;
pub mod lpi;
pub mod route;
pub mod typer;

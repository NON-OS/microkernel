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

//! Loading definition blocks into the namespace: the DSDT, then each SSDT in
//! the order the XSDT lists them (section 5.2.11.2).

mod alias;
mod aml;
mod create;
mod define;
mod exec;
mod fields;
mod reopen;
mod scope_path;
mod scopes;
mod table;
mod walk;

pub use aml::Aml;
pub use exec::LoadExec;
pub use scopes::ParseScopes;
pub use table::Table;

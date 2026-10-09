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

//! `market`: the catalogue the market capsule serves, one listing in full,
//! and asking for an install, from the Terminal. It speaks the wire the
//! Marketplace window speaks (`nonos_market_proto`), so the two read the
//! same answers the same way.

mod call;
mod failure;
mod info;
mod install;
mod list;
mod run;
mod stage_text;
mod uninstall;
mod wrap;

pub use run::run;

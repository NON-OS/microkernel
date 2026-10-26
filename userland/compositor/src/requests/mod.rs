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

//! Every fixed-size NCMP request body, read into what it asks. Pure and
//! apart from the handlers, which hold the IPC and the rights, so each
//! decoder can be driven with hostile bodies on the host.

pub mod area;
pub mod capture;
pub mod place;
pub mod pointer;
pub mod read;
pub mod remove;
pub mod toggles;
pub mod vcp;

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

//! The fact sheet: the EHCI capability and operational registers, the
//! root ports, the queue head and qTD, the legacy support capability in
//! config space, the PCI identity and every bound. Source: the EHCI
//! specification, revision 1.0, as recalled and unverified: it could not
//! be opened where this was written.

pub mod bounds;
pub mod legsup;
pub mod pci;
pub mod port;
pub mod qh;
pub mod qtd;
pub mod regs;

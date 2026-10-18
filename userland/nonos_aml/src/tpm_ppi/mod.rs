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

//! The TPM's Physical Presence Interface, read-only: its _DSM version and
//! which operations firmware says the OS may request. NONOS never writes a
//! PPI request; the functions that submit one are not called here.

mod dsm;
mod read;

pub use dsm::{PPI_UUID, TPM_IDS};
pub use read::{read_ppi, Ppi, MAX_OP};

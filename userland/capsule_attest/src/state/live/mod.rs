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

//! One read of the kernel's process table, which is what every proof this
//! capsule serves is now derived from.
//!
//! It used to answer `OP_PROOF_CAPSULE_LIST` from a table of seventeen names and
//! capability masks typed into the source. That table was accurate on the day it
//! was written and became a fiction the moment a capsule was added, renamed or
//! regranted, and it was served under the word "proof". A proof service whose
//! evidence is a literal is worse than none, because the caller cannot tell.
//!
//! Any token may call `MkProcStat`, but the kernel shows other processes'
//! capability masks only to a holder of AttestRead or ProcessControl, so this
//! capsule holds AttestRead beside IPC and Memory (`Capsule.mk`). It reads
//! masks; it cannot change them.

mod caps;
mod identity;
mod read;
mod types;

pub use caps::{CAP_ADMIN, CAP_DEBUG};
pub use identity::identity;
pub use read::snapshot;
pub use types::Snapshot;

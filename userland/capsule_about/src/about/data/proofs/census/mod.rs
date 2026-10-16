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

//! Who holds authority on this machine, and whether the spawn gate admitted
//! each of them. Pure: the inputs are the process table and the attestation
//! registry as read, so the rules are proven on the host (attest_doc_proofs).
//!
//! The claim it settles is the one that matters for a capability system:
//! every process that holds any capability was admitted by the kernel's gate.
//! Two kinds of process are outside the registry by design and are named
//! rather than hidden. init is the kernel's own. A Linux guest is born with an
//! empty mask (`MkForeignSpawn`) and acts only through the personality that
//! supervises it, which is itself admitted; a capability-free process can
//! reach nothing, so it is counted as sandboxed, not as unproven.

mod counts;
pub mod marks;
mod rule;

pub use counts::Census;
pub use marks::{Admitted, Proc, CAP_NETWORK};
pub use rule::census;

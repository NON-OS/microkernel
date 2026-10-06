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

//! The pure half of nonos.prove: from the verifier's request, the registry
//! transcript, this boot's slots, the endorsement key and the device secret,
//! the statement and the witness, or the invariant that refused them; and the
//! file the proof leaves in. Nothing here calls the kernel, so the host
//! proofs in `userland/prove_proofs` run these files as they ship, mounted
//! one by one; this file re-exports only what the capsule itself calls.

mod enrolled;
mod error;
mod error_text;
mod output;
mod output_read;
mod request;
mod slots;
mod statement;
mod transcript_line;
mod wipe;
mod words;

pub use enrolled::{enrolled, Enrolled, TRANSCRIPT_MAX};
pub use output::encode;
pub use output_read::decode;
pub use request::{parse_request, Request, REQUEST_MAX};
pub use slots::{slots, Slots};
pub use statement::assemble;
pub use wipe::{wipe, wipe_bytes};

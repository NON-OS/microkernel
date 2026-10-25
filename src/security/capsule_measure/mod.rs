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

//! PCR 14: one extend for each distinct capsule the kernel admits, at admission,
//! with the certificate's digest, the digest of what verified it and which
//! kind of check that was, logged so a verifier replays it
//! (docs/design/k1-attest-doc-v4.md, "PCR 14").

mod admit;
mod algs;
mod banks;
mod declared;
pub mod event;
mod extend;
pub mod kind;
mod log;

pub use admit::admit;
pub use extend::admitted;
pub use log::with_events;
